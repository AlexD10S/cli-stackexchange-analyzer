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
    print_global_data(&global_data);
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
    print_team_data(&team_answered);
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

fn print_global_data(global_data: &GlobalAnswers)  {
    println!("------ Global Metrics ------");
    println!();
    println!("Number of questions on this period: {:?} ", global_data.total_questions());
    println!("Unanswered questions on this period: {:?} ", global_data.total_unanswered());
    println!();
}

fn print_team_data(team_answered_questions: &TeamAnswers)  {
    println!("------ Team Metrics ------");
    println!();
    println!("Number of questions answered by team members: {:?} ", team_answered_questions.answers());
    println!("Score of questions answered by team members: {:?} ", team_answered_questions.score());
    println!("Number of questions answered by team members and marked as accepted: {:?} ", team_answered_questions.accepted());
    println!();
}

pub fn print_ratios(global_data: &GlobalAnswers, team_data: &TeamAnswers)  {
    println!("------ Team Ratios ------");
    println!();
    let questions_answered = global_data.total_questions() - global_data.total_unanswered();
    let float_division_total = *global_data.total_unanswered() as f64 / *global_data.total_questions() as f64;
    println!("{:?} % of questions on this period unanswered",  float_division_total * 100 as f64 );

    let float_division_total_team= *team_data.answers() as f64 / *global_data.total_questions() as f64; 
    println!("{:?} % answered by the team over all questions", float_division_total_team * 100 as f64);

    let float_division_answered_team= *team_data.answers() as f64 / questions_answered as f64; 
    println!("{:?} % answered by the team over answered questions", float_division_answered_team * 100 as f64);
    println!();
}