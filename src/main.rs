use clap::Parser;
use serde::{Deserialize};
use dotenv::dotenv;

mod analyzer;
mod utils;
mod api;

use utils::{printer, dates, exporter};
use api::{stackexchange_api};
use analyzer::{core, primitives};

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
    export: bool, //Export the data in a csv file
}

#[tokio::main]
async fn main() {
    // This line loads the environment variables from the ".env" file.
    dotenv().ok(); 

    let args = Cli::parse();

    let period = dates::get_period_in_ms(&args.date_start, &args.date_end);
    let options = primitives::CliOptions { tags: args.tags, individual: args.individual, site: args.site, period};

    let questions = stackexchange_api::get_questions(&options).await;

    let global_data = core::collect_global_data(questions.clone(), &options).await;

    let mut team_data: Option<primitives::Answers> = None;
    if let Some(team_members) = &args.members {
       team_data = Some(core::collect_team_data(questions, team_members, &options).await);
    }
    
    // Print or export data in a csv file
    if args.export {
        exporter::export_data(&args.date_start, &args.date_end, &options, &global_data, &team_data);
    }
    else {
        printer::print_data(&args.date_start, &args.date_end, &options, &global_data, &team_data);
    }
    
}



