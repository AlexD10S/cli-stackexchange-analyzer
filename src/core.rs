use crate::{primitives::{APIResponse, TeamAnswers, GlobalAnswers}, api};

pub async fn collect_global_data(questions: &APIResponse) -> GlobalAnswers  {
    let total_questions = questions.items.len();
    let mut total_unanswered = 0;
    for question in &questions.items {
        if !question.is_answered.unwrap() {
            total_unanswered += 1;
        }
    }
    let global_data =  GlobalAnswers::new(total_questions, total_unanswered);
    return global_data;
}

pub async fn collect_team_data(questions: &APIResponse,  site: &String, members: &Vec<u32>) -> TeamAnswers  {
    let mut team_answered =  TeamAnswers::new(0,0,0);
    for question in &questions.items {
        if question.is_answered.unwrap() {
            let answers: APIResponse = api::get_answers(question.question_id, site).await;
            team_answered = team_answered.question_answered(parse_answers(answers, members));
        }
    }
    return team_answered
}

fn parse_answers(answers: APIResponse, team_members: &Vec<u32>) ->  TeamAnswers {
    let mut team_answered =  TeamAnswers::new(0,0,0);
    for answer in &answers.items {
        if team_members.contains(&answer.owner.user_id)  {
            let aux = TeamAnswers::new(
                1, answer.score, answer.is_accepted.unwrap_or(false) as u32
            );
            team_answered = team_answered.question_answered(aux);
        }
    }
    return team_answered;
}