use crate::{
    primitives::{TeamAnswers, GlobalData, Tag, MemberAnswer, Answers, CliOptions}, 
    dates::get_epoch_in_hr
};
use piechart::{Chart, Color, Data};

const NUMBER_OF_HOT_TAGS: usize = 3;
const TIMER_EMOJI: char = '\u{23F2}';
const HOT_EMOJI: char = '\u{1F525}';

pub fn print_data(date_start: &String, date_end: &String, options: &CliOptions, global_data: &GlobalData, answers: &Option<Answers> )  {
    print_title(&date_start, &date_end, &options.site);
    print_global_data(&global_data);
    
    if let Some(team_data) = &answers {
        print_team_data(&team_data.team_answers());

        if options.individual {
           print_individual_data(&team_data.individual_answers());
        }

        print_ratios(&global_data, &team_data.team_answers());
        print_response_times(&team_data);
    }

    if options.tags {
        print_tags(&global_data);
    }
}

fn print_title(date_start: &String, date_end: &String, site: &String)  {
    println!("-- Questions on {} from {} to {} --", &site, &date_start, &date_end);
    println!();
}

fn print_global_data(global_data: &GlobalData)  {
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

fn print_ratios(global_data: &GlobalData, team_data: &TeamAnswers)  {
    println!("------ Team Ratios ------");
    println!();
    let float_division_total = (*global_data.total_unanswered() as f32 / *global_data.total_questions() as f32) * 100 as f32;
    let float_division_total_team= (*team_data.answers() as f32 / *global_data.total_questions() as f32) * 100 as f32;
    let data = vec![
        Data { label: "Team Answers".into(), value: float_division_total_team, color: Some(Color::Blue.into()), fill: '•' },
        Data { label: "Unanswered".into(), value: float_division_total, color: Some(Color::Red.into()), fill: '▪' },
        Data { label: "Rest".into(), value: (100 as f32 - float_division_total_team - float_division_total), color: Some(Color::Yellow.into()), fill: '▴' },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
    println!();
}

fn print_response_times(answers: &Answers)  {
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

fn print_individual_data(team_answered_questions: &Vec<MemberAnswer>)  {
    println!("------ Individual Metrics ------");
    println!();
    let mut sorted_list = team_answered_questions.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for member in sorted_list{ 
        println!("User {:?} -- Questions: {:?}", member.user_id, member.count); 
    }
    println!();
}

fn print_tags(global_data: &GlobalData,)  {
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