use clap::Parser;
use serde::{Deserialize};

mod primitives;
mod api;
mod utils;
mod core;


#[derive(Parser, Deserialize, Debug)]
struct Cli {
    site: String,
    date_start: String,
    date_end: String,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    members: Option<Vec<u32>>

}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let timestamp_start = utils::get_epoch_ms(&args.date_start);
    let timestamp_end = utils::get_epoch_ms(&args.date_end);

    let questions = api::get_questions(timestamp_start, timestamp_end, &args.site).await;
    println!("-- Questions on {} from {} to {} --", &args.site, &args.date_start, &args.date_end);
    println!();
    
    let global_data = core::collect_global_data(&questions).await;

    if let Some(team_members) = &args.members {
        let team_data = core::collect_team_data(&questions, &args.site, team_members).await;
        core::print_ratios(&global_data, &team_data);
    }

    println!("---------------------");
}



