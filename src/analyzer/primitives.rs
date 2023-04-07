// Primitives for the CLI program
use serde::{Deserialize, Serialize};

use crate::utils::dates::Period;
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CliOptions {
    pub tags: bool,
    pub individual: bool,
    pub site: String,
    pub period: Period,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalData {
    total_questions: usize,
    total_unanswered: usize,
    tags_total: Vec<Tag>,
    tags_unanswered: Vec<Tag>,
}
impl GlobalData {
    pub fn new(
        total_questions: usize,
        total_unanswered: usize,
        tags_total: Vec<Tag>,
        tags_unanswered: Vec<Tag>,
    ) -> Self {
        GlobalData {
            total_questions,
            total_unanswered,
            tags_total,
            tags_unanswered,
        }
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
    score: i32, // Score can be negative
    accepted: u32,
}
impl TeamAnswers {
    pub fn new(answers: u32, score: i32, accepted: u32) -> Self {
        TeamAnswers {
            answers,
            score,
            accepted,
        }
    }
    pub fn answers(&self) -> &u32 {
        &self.answers
    }
    pub fn score(&self) -> &i32 {
        &self.score
    }
    pub fn accepted(&self) -> &u32 {
        &self.accepted
    }
    pub fn question_answered(&self, answer: TeamAnswers) -> TeamAnswers {
        let new_one = TeamAnswers {
            answers: self.answers + answer.answers(),
            score: self.score + answer.score(),
            accepted: self.accepted + answer.accepted(),
        };
        return new_one;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answers {
    team_answers: TeamAnswers,
    individual_answers: Vec<MemberAnswer>,
    time_response_questions: Vec<ResponseTime>,
}
impl Answers {
    pub fn new(
        team_answers: TeamAnswers,
        individual_answers: Vec<MemberAnswer>,
        time_response_questions: Vec<ResponseTime>,
    ) -> Self {
        Answers {
            team_answers,
            individual_answers,
            time_response_questions,
        }
    }
    pub fn team_answers(&self) -> &TeamAnswers {
        &self.team_answers
    }
    pub fn individual_answers(&self) -> &Vec<MemberAnswer> {
        &self.individual_answers
    }
    pub fn time_response_questions(&self) -> &Vec<ResponseTime> {
        &self.time_response_questions
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseTime {
    creation_date: u64,
    response_date: u64,
    team_answered: bool,
}
impl ResponseTime {
    pub fn new(creation_date: u64, response_date: u64, team_answered: bool) -> Self {
        ResponseTime {
            creation_date,
            response_date,
            team_answered,
        }
    }
    pub fn set_response_date(&mut self, new_response_date_value: u64) {
        self.response_date = new_response_date_value;
    }
    pub fn set_team_answered(&mut self, new_team_answered_value: bool) {
        self.team_answered = new_team_answered_value;
    }
    pub fn get_team_answered(&self) -> bool {
        self.team_answered
    }
    pub fn time_response(&self) -> u64 {
        self.response_date - self.creation_date
    }
}
