use reqwest::{ header::{CONTENT_TYPE, ACCEPT}};
use crate::{primitives::APIResponse};


pub async fn get_questions(timestamp_start: i64, timestamp_end: i64, site: &String) ->  APIResponse {
    let url = format!(
        "https://api.stackexchange.com/2.3/questions?fromdate={timestamp_start}&todate={timestamp_end}&site={site}"
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
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

pub async fn get_answers(question_id: u128, site: &String) ->  APIResponse {
    let url = format!(
        "https://api.stackexchange.com/2.3/questions/{question_id}/answers?site={site}"
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
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