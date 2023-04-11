use crate::{
    api::dtos::{APIResponse, Item},
    primitives::CliOptions,
};
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub async fn get_questions(options: &CliOptions) -> Vec<Item> {
    // .expect("error message") in case the API KEY is mandatory, but if is not there just empty space
    let api_key = std::env::var("API_KEY").unwrap_or("".to_string());

    let mut questions: Vec<Item> = Vec::new();

    let mut page = 1;
    let mut api_response = query_questions(&options, &api_key, page).await;
    questions.extend(api_response.items);
    // The while here is for pagination (the api just returns a max of 100 items)
    while api_response.has_more && api_response.quota_remaining > 0 {
        page += 1;
        api_response = query_questions(&options, &api_key, page).await;
        questions.extend(api_response.items);
    }
    questions
}

async fn query_questions(options: &CliOptions, api_key: &String, page: i32) -> APIResponse {
    let url = format!(
        "https://api.stackexchange.com/2.3/questions?fromdate={timestamp_start}&todate={timestamp_end}&site={site}&pagesize={size}&key={key}&page={page}",
        timestamp_start = options.period.timestamp_start,
        timestamp_end = options.period.timestamp_end,
        site = &options.site,
        size = 100, // By default the limit is 30, increase to 100
        key = api_key,
        page = page //Increase to get more than a 100
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

pub async fn get_team_answers(members: &Vec<u32>, options: &CliOptions) -> Vec<Item> {
    // .expect("error message") in case the API KEY is mandatory, but if is not there just empty space
    let api_key = std::env::var("API_KEY").unwrap_or("".to_string());

    let mut answers: Vec<Item> = Vec::new();

    let mut page = 1;
    let mut members_list: String = members.iter().map( |&id| id.to_string() + ";").collect(); 
    members_list.pop();
    let mut api_response = query_answers(&members_list, &options, &api_key, page).await;
    answers.extend(api_response.items);
    // The while here is for pagination (the api just returns a max of 100 items)
    while api_response.has_more && api_response.quota_remaining > 0 {
        page += 1;
        api_response = query_answers(&members_list, &options, &api_key, page).await;
        answers.extend(api_response.items);
    }
    answers
}

async fn query_answers(members: &String, options: &CliOptions, api_key: &String, page: i32) -> APIResponse {
    // Notice don't need the todate here, because we will compare only the answers of the questions retrieved above
    let url = format!(
        "https://api.stackexchange.com/2.3/users/{members}/answers?fromdate={timestamp_start}&site={site}&pagesize={size}&key={key}&page={page}",
        timestamp_start = options.period.timestamp_start,
        site = &options.site,
        size = 100, // By default the limit is 30, increase to 100
        key = api_key,
        page = page //Increase to get more than a 100
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

/// Hardly impossible to have more than a 100 answers in a question, so no need for pagination in this method
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
