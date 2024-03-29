use crate::primitives::{MetricAnswers, CliOptions, MetricsQuestions, MemberAnswers};
use csv;
use std::error::Error;
use std::fs::File;

const NUMBER_OF_HOT_TAGS: usize = 3;

pub fn export_data(
    date_start: &String,
    date_end: &String,
    options: &CliOptions,
    global_data: &MetricsQuestions,
    answers: &Option<MetricAnswers>,
) {
    //By default export it here
    let path = "./data_exported.csv";

    if let Err(e) = export_data_to_csv(
        date_start,
        date_end,
        options,
        global_data,
        answers,
        path,
    ) {
        eprintln!("Error exporting the data into a file: {e}")
    } else {
        println!("Data exported in file {path:?}")
    }
}

fn export_data_to_csv(
    date_start: &String,
    date_end: &String,
    options: &CliOptions,
    global_data: &MetricsQuestions,
    answers: &Option<MetricAnswers>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    export_title(&mut writer, date_start, date_end, options)
        .map_err(|err| eprintln!("{err:?}"))
        .ok();

    export_global_data(&mut writer, global_data)
        .map_err(|err| eprintln!("{err:?}"))
        .ok();

    if let Some(team_data) = &answers {
        export_team_data(&mut writer, team_data)
            .map_err(|err| eprintln!("{err:?}"))
            .ok();

        if options.individual {
            export_individual_team_data(&mut writer, team_data)
                .map_err(|err| eprintln!("{err:?}"))
                .ok();
        }
        export_ratios(&mut writer, global_data, team_data)
            .map_err(|err| eprintln!("{err:?}"))
            .ok();

        export_time_response(&mut writer, team_data)
            .map_err(|err| eprintln!("{err:?}"))
            .ok();
    }

    if options.tags {
        export_tags(&mut writer, global_data)
            .map_err(|err| eprintln!("{err:?}"))
            .ok();
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}

fn export_title(
    writer: &mut csv::Writer<File>,
    date_start: &String,
    date_end: &String,
    options: &CliOptions,
) -> Result<(), Box<dyn Error>> {
    writer.write_record(["Site", &options.site, "", ""])?;
    writer.write_record(["Dates", date_start, date_end, ""])?;
    Ok(())
}

fn export_global_data(
    writer: &mut csv::Writer<File>,
    global_data: &MetricsQuestions,
) -> Result<(), Box<dyn Error>> {
    writer.write_record([
        "Global Data",
        "Number of Questions",
        "Unanswered Questions",
        "",
    ])?;

    writer.write_record([
        "",
        &global_data.total_questions().to_string(),
        &global_data.total_unanswered().to_string(),
        "",
    ])?;
    Ok(())
}

fn export_team_data(
    writer: &mut csv::Writer<File>,
    team_data: &MetricAnswers,
) -> Result<(), Box<dyn Error>> {
    let team_metrics = &team_data.calculate_team_metrics();
    writer.write_record(["Team Data", "Questions Answered", "Score", "Accepted"])?;

    writer.write_record([
        "",
        &team_metrics.answers().to_string(),
        &team_metrics.score().to_string(),
        &team_metrics.accepted().to_string(),
    ])?;
    Ok(())
}

fn export_individual_team_data(
    writer: &mut csv::Writer<File>,
    team_data: &MetricAnswers,
) -> Result<(), Box<dyn Error>> {
    writer.write_record(["Individual Data", "", "", ""])?;
    let team_answered_questions: &Vec<MemberAnswers> = team_data.individual_answers();
    let mut sorted_list = team_answered_questions.clone();
    sorted_list.sort_by(|a, b| b.metrics.answers().cmp(a.metrics.answers()));
    for member in sorted_list {
        writer.write_record([
            "Member",
            &member.user_id.to_string(),
            "Answers",
            &member.metrics.answers().to_string(),
        ])?;
    }
    Ok(())
}

fn export_ratios(
    writer: &mut csv::Writer<File>,
    global_data: &MetricsQuestions,
    team_data: &MetricAnswers,
) -> Result<(), Box<dyn Error>> {
    let questions_answered = global_data.total_questions() - global_data.total_unanswered();
    let float_division_total =
        *global_data.total_unanswered() as f64 / *global_data.total_questions() as f64;

    let team_metrics = &team_data.calculate_team_metrics();
    let answers = team_metrics.answers();
    let float_division_total_team = *answers as f64 / *global_data.total_questions() as f64;
    let float_division_answered_team = *answers as f64 / questions_answered as f64;

    writer.write_record([
        "Ratios",
        "Answered by Team",
        "Unanswered",
        "Answered by Team over answered",
    ])?;

    writer.write_record([
        "%",
        &(float_division_total_team * 100_f64).to_string(),
        &(float_division_total * 100_f64).to_string(),
        &(float_division_answered_team * 100_f64).to_string(),
    ])?;
    Ok(())
}

fn export_time_response(
    writer: &mut csv::Writer<File>,
    team_data: &MetricAnswers,
) -> Result<(), Box<dyn Error>> {
    writer.write_record(["Response Time", "", "Team Average", ""])?;

    let team_metrics = &team_data.calculate_team_metrics();
    let average_team_response = &team_data.time_response_questions(*team_metrics.answers());
    
    writer.write_record([
        "",
        "",
        &(average_team_response).to_string(),
        "",
    ])?;
    Ok(())
}

fn export_tags(
    writer: &mut csv::Writer<File>,
    global_data: &MetricsQuestions,
) -> Result<(), Box<dyn Error>> {
    writer.write_record(["Total Tags", "Tag", "Number", ""])?;
    let mut sorted_list = global_data.tags_total().clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    for n in 0..NUMBER_OF_HOT_TAGS {
        writer.write_record([
            "",
            &sorted_list[n].name.to_string(),
            &sorted_list[n].count.to_string(),
            "",
        ])?;
    }

    writer.write_record(["Unanswered Tags", "Tag", "Number", ""])?;
    let mut sorted_list_unanswered = global_data.tags_unanswered().clone();
    sorted_list_unanswered.sort_by(|a, b| b.count.cmp(&a.count));
    for n in 0..NUMBER_OF_HOT_TAGS {
        writer.write_record([
            "",
            &sorted_list_unanswered[n].name.to_string(),
            &sorted_list_unanswered[n].count.to_string(),
            "",
        ])?;
    }
    Ok(())
}
