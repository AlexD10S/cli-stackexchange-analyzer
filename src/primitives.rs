use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub tags: bool,
    pub individual: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIResponse {
    pub items: Vec<Item>,
    pub has_more: bool,
    pub quota_max: u64,
    pub quota_remaining: u64
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
    pub score: u32,
    pub last_activity_date: u128,
    pub creation_date: u128,
    pub last_edit_date: Option<u128>,
    pub question_id: u128,
    pub content_license: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    account_id: u32,
    reputation: u64,
    pub user_id: u32,
    user_type: String,
    profile_image: String,
    display_name: String,
    link: String,
    accept_rate: Option<u64>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalAnswers {
    total_questions: usize,
    total_unanswered: usize,
    tags_total: Vec<Tag>,
    tags_unanswered: Vec<Tag>,
}
impl GlobalAnswers {
    pub fn new(total_questions: usize, total_unanswered: usize, tags_total: Vec<Tag>, tags_unanswered: Vec<Tag>) -> Self {
        GlobalAnswers { total_questions, total_unanswered, tags_total, tags_unanswered}
    }
    pub fn total_questions(&self) -> &usize {
        &self.total_questions
    }
    pub fn total_unanswered(&self) -> &usize {
        &self.total_unanswered
    }
    pub fn tags_total(&self) -> &Vec<Tag> {
        &self.tags_total
    }
    pub fn tags_unanswered(&self) -> &Vec<Tag> {
        &self.tags_unanswered
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberAnswer {
    pub user_id: u32,
    pub count: u32,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamAnswers {
    answers: u32,
    score: u32,
    accepted: u32,
}
impl TeamAnswers {
    pub fn new(answers: u32, score: u32, accepted: u32) -> Self {
        TeamAnswers { answers, score, accepted}
    }
    pub fn answers(&self) -> &u32 {
        &self.answers
    }
    pub fn score(&self) -> &u32 {
        &self.score
    }
    pub fn accepted(&self) -> &u32 {
        &self.accepted
    }
    pub fn question_answered(&self, answer: TeamAnswers) -> TeamAnswers {
        let new_one =  TeamAnswers { 
            answers: self.answers + answer.answers(), 
            score: self.score + answer.score(),
            accepted: self.accepted + answer.accepted(),
        };
        return new_one;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answers {
    pub team_answers: TeamAnswers,
    pub individual_answers: Vec<MemberAnswer>,
}
impl Answers {
    pub fn new(team_answers: TeamAnswers, individual_answers: Vec<MemberAnswer>) -> Self {
        Answers { team_answers, individual_answers}
    }
    pub fn team_answers(&self) -> &TeamAnswers {
        &self.team_answers
    }
    pub fn individual_answers(&self) -> &Vec<MemberAnswer> {
        &self.individual_answers
    }
}