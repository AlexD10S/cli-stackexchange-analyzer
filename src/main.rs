use clap::Parser;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Parser)]
struct Cli {
    site: String,
    date_start: String,
    date_end: String,
}
fn main() {
    let args = Cli::parse();
    let timestamp_start = get_epoch_ms(&args.date_start);
    let timestamp_end = get_epoch_ms(&args.date_end);
    println!("site {}", args.site);
    println!("{}", timestamp_start);
    println!("{}", timestamp_end);

}

fn get_epoch_ms(date: &str) -> i64 {
    //let time_now = "23/02/2023";
    let naive_date = NaiveDate::parse_from_str(date, "%d/%m/%Y");
    match naive_date {
        Ok(content) => {  
            let naive_datetime: NaiveDateTime = content.and_hms(0,0,0);
            naive_datetime.timestamp()
        }
        Err(error) => { panic!("Error parsing the date {}, just exit here", error); }
    }
}
