use std::error::Error;
use csv;
use crate::{primitives::{GlobalData,Answers, CliOptions, MemberAnswer}};

const NUMBER_OF_HOT_TAGS: usize = 3;

pub fn export_data(date_start: &String, 
    date_end: &String, 
    options: &CliOptions, 
    global_data: &GlobalData, 
    answers: &Option<Answers>){
    //By default export it here
    let path = "./data_exported.csv";

    if let Err(e) = export_data_to_csv(&date_start, &date_end, &options, &global_data, &answers, &path) {
        eprintln!("Error exporting the data into a file: {}", e)
    }
    else{
        println!("Data exported in file {:?}", path)
    }
}

fn export_data_to_csv(
    date_start: &String, 
    date_end: &String, 
    options: &CliOptions, 
    global_data: &GlobalData, 
    answers: &Option<Answers>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(&[
        "Site",
        &options.site,
        "",
        "",
    ])?;
    writer.write_record(&[
        "Dates",
        date_start,
        date_end,
        "",
    ])?;

    writer.write_record(&[
        "Global Data",
        "Number of Questions",
        "Unanswered Questions",
        "",
    ])?;

    writer.write_record(&[
        "",
        &global_data.total_questions().to_string(),
        &global_data.total_unanswered().to_string(),
        "",
    ])?;

    if let Some(team_data) = &answers {
        writer.write_record(&[
            "Team Data",
            "Questions Answered",
            "Score",
            "Accepted",
        ])?;
    
        writer.write_record(&[
            "",
            &team_data.team_answers().answers().to_string(),
            &team_data.team_answers().score().to_string(),
            &team_data.team_answers().accepted().to_string(),
        ])?;

        if options.individual {
            writer.write_record(&[
                "Individual Data",
                "",
                "",
                "",
            ])?;
            let team_answered_questions: &Vec<MemberAnswer> = &team_data.individual_answers();
            let mut sorted_list = team_answered_questions.clone();
            sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
            for member in sorted_list{ 
                writer.write_record(&[
                    "Member",
                    &member.user_id.to_string(),
                    "Answers",
                    &member.count.to_string(),
                ])?;
            }
        }
        let questions_answered = global_data.total_questions() - global_data.total_unanswered();
        let float_division_total = *global_data.total_unanswered() as f64 / *global_data.total_questions() as f64;
        let float_division_total_team= *team_data.team_answers().answers() as f64 / *global_data.total_questions() as f64; 
        let float_division_answered_team= *team_data.team_answers().answers() as f64 / questions_answered as f64; 

        writer.write_record(&[
            "Ratios",
            "Answered by Team",
            "Unanswered",
            "Answered by Team over answered",
        ])?;
    
        writer.write_record(&[
            "%",
            &(float_division_total_team * 100 as f64).to_string(),
            &(float_division_total * 100 as f64).to_string(),
            &(float_division_answered_team * 100 as f64).to_string(),
        ])?;

        writer.write_record(&[
            "Response Time",
            "Average",
            "Team Average",
            "",
        ])?;

        let mut total_time_response: u64 = 0;
        let mut total_team_time_response: u64 = 0;
        let time_response_questions = team_data.time_response_questions();
        for time_response in time_response_questions {
            if time_response.get_team_answered() {
                total_team_time_response += time_response.time_response();
            }
            total_time_response += time_response.time_response();
        }
        let average_total_response = total_time_response as f64 / time_response_questions.len() as f64;
        let average_team_response = total_team_time_response as f64 / *team_data.team_answers().answers() as f64;
    
        writer.write_record(&[
            "",
            &(average_total_response).to_string(),
            &(average_team_response).to_string(),
           "",
        ])?;
    }

    if options.tags {
        writer.write_record(&[
            "Total Tags",
            "Tag",
            "Number",
            "",
        ])?;
        let mut sorted_list = global_data.tags_total().clone();
        sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
        for n in 0..NUMBER_OF_HOT_TAGS { 
            writer.write_record(&[
                "",
                &sorted_list[n].name.to_string(),
                &sorted_list[n].count.to_string(),
                "",
            ])?;
        }

        writer.write_record(&[
            "Unanswered Tags",
            "Tag",
            "Number",
            "",
        ])?;
        let mut sorted_list_unanswered = global_data.tags_unanswered().clone();
        sorted_list_unanswered.sort_by(|a, b| b.count.cmp(&a.count));
        for n in 0..NUMBER_OF_HOT_TAGS { 
            writer.write_record(&[
                "",
                &sorted_list_unanswered[n].name.to_string(),
                &sorted_list_unanswered[n].count.to_string(),
                "",
            ])?;
        }
    
        
    }


    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}
