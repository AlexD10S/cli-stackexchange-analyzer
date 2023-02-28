use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIResponse {
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