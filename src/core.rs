use crate::{primitives::APIResponse, api};

pub async fn collect_data(questions: APIResponse,  site: &String)  {
    let number_of_questions = questions.items.len();
    let mut unanswered_questions = 0;
    for question in &questions.items {
        if !question.is_answered.unwrap() {
            unanswered_questions += 1;
        }
        else{
            let answers: APIResponse = api::get_answers(question.question_id, site).await;
            parse_answers(answers)
        }
    }
    print_data(number_of_questions, unanswered_questions);
}

fn parse_answers(answers: APIResponse) {
    println!("{:?}", answers);
}

fn print_data(number_of_questions: usize, unanswered_questions: usize)  {
    println!("------ Metrics ------");
    println!("Number of questions on this period: {:?} ", number_of_questions);
    println!("Unanswered questions on this period: {:?} ", unanswered_questions);
    println!("---------------------");
}