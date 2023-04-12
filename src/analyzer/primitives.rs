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
pub struct MetricsQuestions {
    total_questions: usize,
    total_unanswered: usize,
    tags_total: Vec<Tag>,
    tags_unanswered: Vec<Tag>,
}
impl MetricsQuestions {
    pub fn new(
        total_questions: usize,
        total_unanswered: usize,
        tags_total: Vec<Tag>,
        tags_unanswered: Vec<Tag>,
    ) -> Self {
        MetricsQuestions {
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
pub struct MetricAnswers {
    individual_answers: Vec<MemberAnswers>,
    time_response_questions: u64,
}
impl MetricAnswers {
    pub fn new(
        individual_answers: Vec<MemberAnswers>,
    ) -> Self {
        MetricAnswers {
            individual_answers,
            time_response_questions: 0,
        }
    }
    pub fn individual_answers(&self) -> &Vec<MemberAnswers> {
        &self.individual_answers
    }
    pub fn add_time_response_questions(&mut self, creation_date: u64, response_date: u64) {
        self.time_response_questions = self.time_response_questions + (response_date - creation_date);
    }
    pub fn time_response_questions(&self, number_answers: u32) -> f64 {
        self.time_response_questions as f64 / number_answers as f64
    }
    pub fn add_answer(&mut self, answer: MemberAnswers){
        if let Some(index) =  self.individual_answers.iter().position(|individual_answer| individual_answer.user_id == answer.user_id) {
            self.individual_answers[index].metrics = self.individual_answers[index].metrics.add_question_answered(&answer.metrics);
        }
        else{
            self.individual_answers.push(answer);
        }
    }
    pub fn calculate_team_metrics(&self) -> IndividualMetrics{
        let mut team_metrics = IndividualMetrics::new(0,0,0);
        for individual_metrics in &self.individual_answers {
            team_metrics = team_metrics.add_question_answered(&individual_metrics.metrics);
        }
        return team_metrics;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberAnswers {
    pub user_id: u32,
    pub metrics: IndividualMetrics,
}
impl MemberAnswers {
    pub fn new(user_id: u32, metrics: IndividualMetrics) -> Self {
        MemberAnswers {
            user_id,
            metrics
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndividualMetrics {
    answers: u32,
    score: i32, // Score can be negative
    accepted: u32,
}
impl IndividualMetrics {
    pub fn new(answers: u32, score: i32, accepted: u32) -> Self {
        IndividualMetrics {
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
    pub fn add_question_answered(&self, answer: &IndividualMetrics) -> IndividualMetrics {
        let new_one = IndividualMetrics {
            answers: self.answers + answer.answers(),
            score: self.score + answer.score(),
            accepted: self.accepted + answer.accepted(),

        };
        return new_one;
    }
}