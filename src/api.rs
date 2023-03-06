use reqwest::{ header::{CONTENT_TYPE, ACCEPT}};
use crate::{primitives::{APIResponse, Item}, utils::Period};


pub async fn get_questions(period: &Period, site: &String) -> Vec<Item> {
    // .expect("error message") in case the API KEY is mandatory, but if is not there just empty space
    let api_key = std::env::var("API_KEY").unwrap_or("".to_string());
    let mut page = 1;
    let mut questions: Vec<Item> = Vec::new();
    let mut api_response = query_questions(period, site, &api_key, page).await;
    questions.extend(api_response.items);
    while api_response.has_more && api_response.quota_remaining > 0 {
        page += 1;
        api_response = query_questions(period, site, &api_key, page).await;
        questions.extend(api_response.items);
    }
    questions
}

async fn query_questions(period: &Period, site: &String, api_key: &String, page: i32) -> APIResponse {
    let url = format!(
        "https://api.stackexchange.com/2.3/questions?fromdate={timestamp_start}&todate={timestamp_end}&site={site}&pagesize=100&key={key}&page={page}",
        timestamp_start = period.timestamp_start,
        timestamp_end = period.timestamp_end,
        key = api_key,
        page = page
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

pub async fn get_answers(question_id: u128, site: &String) -> APIResponse {
    // .expect("error message") in case the API KEY is mandatory, but if is not there just empty space
    let api_key = std::env::var("API_KEY").unwrap_or("".to_string());
    let url = format!(
        "https://api.stackexchange.com/2.3/questions/{question_id}/answers?site={site}&pagesize=100&key={api_key}"
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