use clap::Parser;
use serde::{Deserialize};
use dotenv::dotenv;

mod primitives;
mod dtos;
mod api;
mod dates;
mod core;
mod metrics;
mod utils;

#[derive(Parser, Deserialize, Debug)]
struct Cli {
    site: String,  // Site to query
    date_start: String, //Date to start with dd/mm/YYYY format 
    date_end: String, //Date to end with dd/mm/YYYY format 
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    members: Option<Vec<u32>>, //List of members of your team 
    #[clap(long, short, action)]
    tags: bool, //Collect tags info
    #[clap(long, short, action)]
    individual: bool, //Collect individual team members info
    #[clap(long, short, action)]
    unanswered: bool //Analyse Unanswered questions
}

#[tokio::main]
async fn main() {
    // This line loads the environment variables from the ".env" file.
    dotenv().ok(); 

    let args = Cli::parse();

    let period = dates::get_period_in_ms(&args.date_start, &args.date_end);
    let options = primitives::Options { tags: args.tags, individual: args.individual, unanswered: args.unanswered};

    let questions = api::get_questions(&period, &args.site).await;

    metrics::print_title(&args.date_start, &args.date_end, &args.site);
    
    let global_data = core::collect_global_data(questions.clone(), &options).await;
    metrics::print_global_data(&global_data);

    if let Some(team_members) = &args.members {
        let team_data = core::collect_team_data(questions, &args.site, team_members, &options).await;
        metrics::print_team_data(&team_data.team_answers());

        if options.individual {
            metrics::print_individual_data(&team_data.individual_answers());
        }

        metrics::print_ratios(&global_data, &team_data.team_answers());
        metrics::print_response_times(&team_data);

        if options.unanswered {
            metrics::print_unanswered_analysed(&team_data.unanswered_questions(), &global_data);
        }
    }

    if options.tags {
        metrics::print_tags(&global_data);
    }
    
}



