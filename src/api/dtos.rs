// Objects to parse the data coming from the StackExchange API
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIResponse {
    pub items: Vec<Item>,
    pub has_more: bool,
    pub quota_max: u64,
    pub quota_remaining: u64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub tags: Option<Vec<String>>,
    pub owner: Owner,
    pub is_answered: Option<bool>,
    pub is_accepted: Option<bool>,
    pub view_count: Option<u32>,
    pub accepted_answer_id: Option<u64>,
    pub answer_id: Option<u64>,
    pub answer_count: Option<u32>,
    pub score: i32,
    pub last_activity_date: u64,
    pub creation_date: u64,
    pub last_edit_date: Option<u64>,
    pub question_id: u128,
    pub content_license: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    account_id: Option<u32>,
    reputation: Option<u64>,
    pub user_id: Option<u32>,
    user_type: String,
    profile_image: Option<String>,
    display_name: Option<String>,
    link: Option<String>,
    accept_rate: Option<u64>,
}
