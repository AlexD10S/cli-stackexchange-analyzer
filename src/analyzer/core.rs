use crate::{primitives::{
    TeamAnswers, GlobalAnswers, Options, Answers, ResponseTime},
    api::dtos::{APIResponse, Item}, 
    api::stackexchange_api, 
    utils::utils::{parse_answers, add_tags}
};

pub async fn collect_global_data(questions: Vec<Item>, options: &Options) -> GlobalAnswers  {
    let total_questions = questions.len();
    let mut total_unanswered = 0;
    let mut tags_total = Vec::new();
    let mut tags_unanswered = Vec::new();
   
    for question in &questions {
        if !question.is_answered.unwrap() && question.answer_count.unwrap() == 0 {
            total_unanswered += 1;
            if question.tags.is_some() && options.tags { 
                add_tags(&mut tags_unanswered, question.tags.as_ref().unwrap());
            }
        }
        if options.tags {
            add_tags(&mut tags_total, question.tags.as_ref().unwrap());
        }
    }
    let global_data =  GlobalAnswers::new(total_questions, total_unanswered, tags_total, tags_unanswered);
    return global_data;
}

pub async fn collect_team_data(questions: Vec<Item>, site: &String, members: &Vec<u32>, options: &Options) -> Answers  {
    let mut answers_by_member = Vec::new();
    let mut team_answered =  TeamAnswers::new(0,0,0);
    let mut time_response_questions: Vec<ResponseTime> = Vec::new();

    for question in &questions {
        if question.is_answered.unwrap() || (!question.is_answered.unwrap() && question.answer_count.unwrap() > 0) {
            let mut response_time: ResponseTime = ResponseTime::new(question.creation_date, 0, false);

            let answers: APIResponse = stackexchange_api::get_answers(question.question_id, site).await;

            let team_answers: TeamAnswers = parse_answers(answers, &mut answers_by_member, members, &mut response_time, options);
            team_answered = team_answered.question_answered(team_answers);

            time_response_questions.push(response_time);
        }
    }
    let answers: Answers = Answers::new(team_answered, answers_by_member, time_response_questions);
    return answers
}