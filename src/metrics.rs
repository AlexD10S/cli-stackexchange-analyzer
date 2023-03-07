use crate::{primitives::{TeamAnswers, GlobalAnswers, Tag, MemberAnswer, ResponseTime}};

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

pub fn print_response_times(time_response_questions: &Vec<ResponseTime>)  {
    println!("------ Team Response Times ------");
    println!();
    println!("{:?} ", time_response_questions);
    println!();
}

pub fn print_individual_data(team_answered_questions: &Vec<MemberAnswer>)  {
    println!("------ Individual Metrics ------");
    println!();
    let mut sorted_list = team_answered_questions.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for member in  sorted_list{ 
        println!("User {:?} -- Questions: {:?}", member.user_id, member.count); 
    }
    println!();
}

pub fn print_tags(global_data: &GlobalAnswers,)  {
    const HOT_EMOJI: char = '\u{1F525}';
    println!("------{:?} {:?} Hot Tags {:?} {:?}------", HOT_EMOJI, HOT_EMOJI, HOT_EMOJI, HOT_EMOJI );
    println!();
    println!("--- Total tags ---");
    print_list(&global_data.tags_total());
    println!();
    println!("--- Unanswered tags ---");
    print_list(&global_data.tags_unanswered());
    println!();
}

fn print_list(tags_vec: &Vec<Tag>)  {
    const NUMBER_OF_HOT_TAGS: usize = 3;

    let mut sorted_list = tags_vec.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for n in 0..NUMBER_OF_HOT_TAGS { 
        println!("{:?} -- Number of questions with this Tag: {:?}", sorted_list[n].name.to_string(), sorted_list[n].count); 
    }
}