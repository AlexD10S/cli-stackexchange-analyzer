use crate::{primitives::{
    TeamAnswers, GlobalAnswers, Options, Answers, ResponseTime, UnanswerQuestions},
    dtos::{APIResponse, Item}, 
    api, 
    utils::{parse_answers, add_tags}
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
    let mut unanswered_questions: Vec<UnanswerQuestions> = Vec::new();

    for question in &questions {
        if question.is_answered.unwrap() || (!question.is_answered.unwrap() && question.answer_count.unwrap() > 0) {
            let mut response_time: ResponseTime = ResponseTime::new(question.creation_date, 0, false);

            let answers: APIResponse = api::get_answers(question.question_id, site).await;

            let team_answers: TeamAnswers = parse_answers(answers, &mut answers_by_member, members, &mut response_time, options);
            team_answered = team_answered.question_answered(team_answers);

            time_response_questions.push(response_time);
        }
        // if !question.is_answered.unwrap() && options.unanswered {
        //     let unanswered_question: UnanswerQuestions = analyse_unanswered_question(question.question_id, site, members, options).await;
        //     unanswered_questions.push(unanswered_question);
        // }
    }
    let answers: Answers = Answers::new(team_answered, answers_by_member, time_response_questions, unanswered_questions);
    return answers
}

async fn analyse_unanswered_question(question_id: u128, site: &String, team_members: &Vec<u32>, options: &Options) -> UnanswerQuestions {
    let answers: APIResponse = api::get_answers(question_id, site).await;
    let mut answered = false;
    let mut answered_by_team = false;
    let mut user_id: u32 = 0;
    for answer in &answers.items {
        answered = true;
        if team_members.contains(&answer.owner.user_id.unwrap_or(0))  {
            answered_by_team = true;
            if options.individual {
                user_id = answer.owner.user_id.unwrap_or(0);
            }
        }
    }
    UnanswerQuestions {answered, answered_by_team, user_id}
}