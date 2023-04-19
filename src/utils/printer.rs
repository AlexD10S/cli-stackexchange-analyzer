use crate::{
    primitives::{MetricAnswers, CliOptions, MetricsQuestions, MemberAnswers, Tag, IndividualMetrics},
    utils::charts::{display_chart_ratios, display_chart_tags},
    utils::dates::get_epoch_in_hr,
};

const TIMER_EMOJI: char = '\u{23F2}';
const HOT_EMOJI: char = '\u{1F525}';

pub fn print_data(
    date_start: &String,
    date_end: &String,
    options: &CliOptions,
    global_data: &MetricsQuestions,
    answers: &Option<MetricAnswers>,
) {
    print_title(date_start, date_end, &options.site);
    print_global_data(global_data);

    if let Some(answers_metrics) = &answers {
        let team_metrics = &answers_metrics.calculate_team_metrics();
        print_team_data(team_metrics);

        if options.individual {
            print_individual_data(answers_metrics.individual_answers());
        }

        print_ratios(global_data, team_metrics);
        print_response_times(answers_metrics, *team_metrics.answers());
    }

    if options.tags {
        print_tags(global_data);
    }
}

fn print_title(date_start: &String, date_end: &String, site: &String) {
    println!(
        "-- Questions on {} from {} to {} --",
        &site, &date_start, &date_end
    );
    println!();
}

fn print_global_data(global_data: &MetricsQuestions) {
    println!("------ Global Metrics ------");
    println!();
    println!(
        "Number of questions on this period: {:?} ",
        global_data.total_questions()
    );
    println!(
        "Unanswered questions on this period: {:?} ",
        global_data.total_unanswered()
    );
    println!();
}

fn print_team_data(team_answered_questions: &IndividualMetrics) {
    println!("------ Team Metrics ------");
    println!();
    println!(
        "Number of questions answered by team members: {:?} ",
        team_answered_questions.answers()
    );
    println!(
        "Score of questions answered by team members: {:?} ",
        team_answered_questions.score()
    );
    println!(
        "Number of questions answered by team members and marked as accepted: {:?} ",
        team_answered_questions.accepted()
    );
    println!();
}

fn print_ratios(global_data: &MetricsQuestions, team_data: &IndividualMetrics) {
    println!("------ Team Ratios ------");
    println!();
    let float_division_total = (*global_data.total_unanswered() as f32
        / *global_data.total_questions() as f32)
        * 100_f32;
    let float_division_total_team =
        (*team_data.answers() as f32 / *global_data.total_questions() as f32) * 100_f32;

    display_chart_ratios(float_division_total, float_division_total_team);
    println!();
}

fn print_response_times(answers: &MetricAnswers, number_answers: u32) {
    println!(
        "------{TIMER_EMOJI:?} {TIMER_EMOJI:?} Team Response Times {TIMER_EMOJI:?} {TIMER_EMOJI:?}------"
    );
    println!();
    let average_team_response = answers.time_response_questions(number_answers);
    println!(
        "The average time of team response is around {:?} hours",
        get_epoch_in_hr(average_team_response)
    );

    println!();
}

fn print_individual_data(team_answered_questions: &Vec<MemberAnswers>) {
    println!("------ Individual Metrics ------");
    println!();
    let mut sorted_list = team_answered_questions.clone();
    sorted_list.sort_by(|a, b| b.metrics.answers().cmp(a.metrics.answers()));
    for member in sorted_list {
        println!("User {:?} -- Questions: {:?}", member.user_id, member.metrics.answers());
    }
    println!();
}

fn print_tags(global_data: &MetricsQuestions) {
    println!(
        "------{HOT_EMOJI:?} {HOT_EMOJI:?} Hot Tags {HOT_EMOJI:?} {HOT_EMOJI:?}------"
    );
    println!();
    if !global_data.tags_total().is_empty() {
        println!("--- Total top tags ---");
        println!();
        print_list(global_data.tags_total());
        println!();
    }
    if !global_data.tags_unanswered().is_empty() {
        println!("--- Unanswered top tags ---");
        println!();
        print_list(global_data.tags_unanswered());
        println!();
    }
}

fn print_list(tags_vec: &Vec<Tag>) {
    let mut sorted_list = tags_vec.clone();
    sorted_list.sort_by(|a, b| b.count.cmp(&a.count));
    println!();
    display_chart_tags(&sorted_list);
    println!();
}
