use crate::{
    primitives::{TeamAnswers, GlobalAnswers, Tag, MemberAnswer, Answers, UnanswerQuestions}, 
    dates::get_epoch_in_hr,
    utils::{add_member_response}
};

const NUMBER_OF_HOT_TAGS: usize = 3;
const TIMER_EMOJI: char = '\u{23F2}';
const HOT_EMOJI: char = '\u{1F525}';

pub fn print_title(date_start: &String, date_end: &String, site: &String)  {
    println!("-- Questions on {} from {} to {} --", &site, &date_start, &date_end);
    println!();
}

pub fn print_global_data(global_data: &GlobalAnswers)  {
    println!("------ Global Metrics ------");
    println!();
    println!("Number of questions on this period: {:?} ", global_data.total_questions());
    println!("Unanswered questions on this period: {:?} ", global_data.total_unanswered());
    println!();
}

pub fn print_team_data(team_answered_questions: &TeamAnswers)  {
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

pub fn print_response_times(answers: &Answers)  {
    println!("------{:?} {:?} Team Response Times {:?} {:?}------", TIMER_EMOJI, TIMER_EMOJI, TIMER_EMOJI, TIMER_EMOJI);
    println!();
    let mut total_time_response: u64 = 0;
    let mut total_team_time_response: u64 = 0;
    let time_response_questions = answers.time_response_questions();
    for time_response in time_response_questions {
        if time_response.get_team_answered() {
            total_team_time_response += time_response.time_response();
        }
        total_time_response += time_response.time_response();
    }
    let average_total_response = total_time_response as f64 / time_response_questions.len() as f64;
    println!("The average time of response is around {:?} hours", get_epoch_in_hr(average_total_response));

    let average_team_response = total_team_time_response as f64 / *answers.team_answers().answers() as f64;
    println!("The average time of team response is around {:?} hours", get_epoch_in_hr(average_team_response));  

    println!();
}

pub fn print_individual_data(team_answered_questions: &Vec<MemberAnswer>)  {
    println!("------ Individual Metrics ------");
    println!();
    let mut sorted_list = team_answered_questions.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for member in sorted_list{ 
        println!("User {:?} -- Questions: {:?}", member.user_id, member.count); 
    }
    println!();
}

pub fn print_unanswered_analysed(unanswered_data: &Vec<UnanswerQuestions>, global_data: &GlobalAnswers)  {
    println!("------ Unanswered Questions Analysed------");
    println!();
    println!("From the {:?} Unanswered Questions:", unanswered_data.len()); 
    let mut answered = 0;
    let mut answered_by_team = 0;
    let mut answers_by_member: Vec<MemberAnswer> = Vec::new();
    for unanswered_question in unanswered_data{ 
       if unanswered_question.answered {
        answered += 1;
       }
       if unanswered_question.answered_by_team {
        answered_by_team += 1;
        add_member_response(&mut answers_by_member, &unanswered_question.user_id);
       }
    }
    println!("{:?} has an answer with 0 score", answered);
    println!("In {:?} of them a team member answer it", answered_by_team); 
    let mut sorted_list = answers_by_member.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for member in sorted_list{ 
        println!("User {:?} has answered {:?} of the unanswered(with 0 score) questions", member.user_id, member.count); 
    }
    println!();
    println!("Real Unanswered questions on this period: {:?} ", global_data.total_unanswered() - answered);
    println!();
}

pub fn print_tags(global_data: &GlobalAnswers,)  {
    println!("------{:?} {:?} Hot Tags {:?} {:?}------", HOT_EMOJI, HOT_EMOJI, HOT_EMOJI, HOT_EMOJI );
    println!();
    println!("--- Total tags ---");
    println!();
    print_list(&global_data.tags_total());
    println!();
    println!("--- Unanswered tags ---");
    println!();
    print_list(&global_data.tags_unanswered());
    println!();
}

fn print_list(tags_vec: &Vec<Tag>)  {
    let mut sorted_list = tags_vec.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for n in 0..NUMBER_OF_HOT_TAGS { 
        println!("Tag: {:?}, used in {:?} questions", sorted_list[n].name.to_string(), sorted_list[n].count); 
    }
}