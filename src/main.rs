use clap::Parser;
use chrono::{NaiveDate, NaiveDateTime};
use reqwest::{ Error, header::{CONTENT_TYPE, ACCEPT, HeaderValue, HeaderMap, USER_AGENT, CONTENT_LENGTH}};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct APIResponse {
    items: Vec<Item>,
    has_more: bool,
    quota_max: u64,
    quota_remaining: u64
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Item {
    tags: Vec<String>,
    owner: Owner,
    is_answered: bool,
    view_count: u32,
    accepted_answer_id: Option<u64>,
    answer_count: u32,
    score: u32,
    last_activity_date: u128,
    creation_date: u128,
    last_edit_date: Option<u128>,
    question_id: u128,
    content_license: Option<String>,
    link: String,
    title: String
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Owner {
    account_id: u128,
    reputation: u64,
    user_id: u64,
    user_type: String,
    profile_image: String,
    display_name: String,
    link: String
}


#[derive(Parser)]
struct Cli {
    site: String,
    date_start: String,
    date_end: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let timestamp_start = get_epoch_ms(&args.date_start);
    let timestamp_end = get_epoch_ms(&args.date_end);

    let questions = get_questions(timestamp_start, timestamp_end, &args.site).await;
    println!("-- Questions on {} from {} to {} --", &args.site, &args.date_start, &args.date_end);
    println!("{:?}", questions)
}


async fn get_questions(timestamp_start: i64, timestamp_end: i64, site: &String) ->  APIResponse {
    let url = format!(
        "https://api.stackexchange.com/2.3/questions?fromdate={timestamp_start}&todate={timestamp_end}&site={site}"
    );
    // let url = "https://api.stackexchange.com/2.3/questions/5196?order=desc&sort=activity&site=substrate";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "reqwest")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => return parsed,
                Err(error) => panic!("Error parsing the response: {:?}", error),
            };
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
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
