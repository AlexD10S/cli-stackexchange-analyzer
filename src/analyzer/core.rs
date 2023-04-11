use crate::{primitives::{
    TeamAnswersMetrics, MetricsQuestions, CliOptions, MetricAnswers, MemberAnswer},
    api::dtos::{Item}, 
    utils::parser::{parse_and_add_tags}
};

pub async fn collect_global_data(questions: Vec<Item>, options: &CliOptions) -> MetricsQuestions  {
    let total_questions = questions.len();
    let mut total_unanswered = 0;
    let mut tags_total = Vec::new();
    let mut tags_unanswered = Vec::new();
   
    for question in &questions {
        if !question.is_answered.unwrap() && question.answer_count.unwrap() == 0 {
            total_unanswered += 1;
            if question.tags.is_some() && options.tags { 
                parse_and_add_tags(&mut tags_unanswered, question.tags.as_ref().unwrap());
            }
        }
        if options.tags {
            parse_and_add_tags(&mut tags_total, question.tags.as_ref().unwrap());
        }
    }
    let global_data =  MetricsQuestions::new(total_questions, total_unanswered, tags_total, tags_unanswered);
    return global_data;
}

pub async fn collect_team_data(team_answers: Vec<Item>, questions: Vec<Item>) -> MetricAnswers  {
    let mut metrics: MetricAnswers = MetricAnswers::new(Vec::new());

    for answer in &team_answers {
        // This is to get just the answers of questions collected from the specific period selected.
        if let Some(index) =  questions.iter().position(|question| question.question_id == answer.question_id) {
            let question = &questions[index];
            metrics.add_time_response_questions(question.creation_date, answer.creation_date);
            let answer_metrics = TeamAnswersMetrics::new(1, answer.score, answer.is_accepted.unwrap_or(false) as u32);
            let answer = MemberAnswer::new(answer.owner.user_id.unwrap_or(0), answer_metrics);
            metrics.add_answer(answer);
        }
       
    }
    return metrics
}
