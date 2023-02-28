use clap::Parser;

mod primitives;
mod api;
mod utils;
mod core;


#[derive(Parser)]
struct Cli {
    site: String,
    date_start: String,
    date_end: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let timestamp_start = utils::get_epoch_ms(&args.date_start);
    let timestamp_end = utils::get_epoch_ms(&args.date_end);

    let questions = api::get_questions(timestamp_start, timestamp_end, &args.site).await;
    println!("-- Questions on {} from {} to {} --", &args.site, &args.date_start, &args.date_end);

    core::collect_data(questions, &args.site).await;
}



