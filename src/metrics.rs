use crate::{primitives::{TeamAnswers, GlobalAnswers, Tag, TagsAPIResponse}};

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

pub fn print_tags(tags: &TagsAPIResponse)  {
    println!("------ Hot Tags ------");
    println!("{:?}", &tags.items);
    for tag in &tags.items {
        println!("{:?} - {:?}", tag.name, tag.count);
    }
    println!();
}