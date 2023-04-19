use clap::Parser;
use dotenv::dotenv;
use serde::Deserialize;

mod analyzer;
mod api;
mod utils;

use analyzer::{core, primitives};
use api::stackexchange_api;
use utils::{dates, exporter, printer, tags_handler};

#[derive(Parser, Deserialize, Debug)]
#[command(author, version)]
#[command(about = "CLI tool to get metrics from a stackexchange site built with Rust 🦀.")]
struct Cli {
    /// Site to fetch the data from
    site: String,
    /// Date to start - dd/mm/YYYY format
    date_start: String,
    /// Date to end - dd/mm/YYYY format
    date_end: String, 
    /// List of members of your team
    #[clap(short = 'm', long = "members", value_parser, num_args = 1.., value_delimiter = ' ')]
    members: Option<Vec<u32>>,
    /// Collect tags info
    #[clap(long, short, action)]
    tags: bool, 
    /// Collect individual team members info
    #[clap(long, short, action)]
    individual: bool,
    /// Get the metrics on a specific a tag
    #[clap(short = 'b', long = "by_tag", value_parser, num_args = 1)]
    by_tag: Option<String>,
    /// Export the data in a csv file
    #[clap(long, short, action)]
    export: bool,
}

#[tokio::main]
async fn main() {
    // This line loads the environment variables from the ".env" file.
    dotenv().ok();

    let args = Cli::parse();

    let period = dates::get_period_in_ms(&args.date_start, &args.date_end);

    let options = primitives::CliOptions {
        tags: args.tags,
        individual: args.individual,
        site: args.site,
        period,
    };
    let mut questions = stackexchange_api::get_questions(&options).await;
    if let Some(tag) = &args.by_tag {
        tags_handler::filter_questions_by_tags(&mut questions, tag);
    }
    let global_data = core::collect_global_data(questions.clone(), &options).await;

    let mut team_data: Option<primitives::MetricAnswers> = None;
    if let Some(team_members) = &args.members {
        println!("Analyzing all the questions (Please wait)...");

        let answers = stackexchange_api::get_team_answers(team_members, &options).await;
        team_data = Some(core::collect_team_data(answers, questions).await);
    }
    
    // Print metrics on screen or export the metrics in a csv file
    if args.export {
        exporter::export_data(
            &args.date_start,
            &args.date_end,
            &options,
            &global_data,
            &team_data,
        );
    } else {
        printer::print_data(
            &args.date_start,
            &args.date_end,
            &options,
            &global_data,
            &team_data,
        );
    }
}
