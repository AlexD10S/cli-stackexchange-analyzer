use crate::{primitives::{
    TeamAnswersMetrics, MetricsQuestions, CliOptions, MetricAnswers, ResponseTime},
    api::dtos::{APIResponse, Item}, 
    api::stackexchange_api, 
    utils::parser::{parse_answers, parse_and_add_tags}
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

pub async fn collect_team_data(questions: Vec<Item>, members: &Vec<u32>, options: &CliOptions) -> MetricAnswers  {
    let mut answers_by_member = Vec::new();
    let mut team_answer_metrics =  TeamAnswersMetrics::new(0,0,0);
    let mut time_response_questions: Vec<ResponseTime> = Vec::new();

    for question in &questions {
        if question.is_answered.unwrap() || (!question.is_answered.unwrap() && question.answer_count.unwrap() > 0) {
            let mut response_time: ResponseTime = ResponseTime::new(question.creation_date, 0, false);

            let answers_of_question: APIResponse = stackexchange_api::get_answers(question.question_id, &options.site).await;
            
            let team_answer_in_question: TeamAnswersMetrics = parse_answers(
                answers_of_question,
                &mut answers_by_member, 
                members, 
                &mut response_time, 
                options
            );
            team_answer_metrics = team_answer_metrics.add_question_answered_by_team(team_answer_in_question);

            time_response_questions.push(response_time);
        }
    }
    let answers: MetricAnswers = MetricAnswers::new(team_answer_metrics, answers_by_member, time_response_questions);
    return answers
}