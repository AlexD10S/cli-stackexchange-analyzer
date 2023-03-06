use crate::{primitives::{APIResponse, Item, TeamAnswers, GlobalAnswers, Options, Tag, MemberAnswer, Answers}, api};

pub async fn collect_global_data(questions: Vec<Item>, options: &Options) -> GlobalAnswers  {
    let total_questions = questions.len();
    let mut total_unanswered = 0;
    let mut tags_total = Vec::new();
    let mut tags_unanswered = Vec::new();
   
    for question in &questions {
        if !question.is_answered.unwrap() {
            total_unanswered += 1;
            if question.tags.is_some() && options.tags { 
                add_tags(&mut tags_unanswered, question.tags.as_ref().unwrap());
            }
        }
        if options.tags {
            add_tags(&mut tags_total, question.tags.as_ref().unwrap());
        }
    }
    let global_data =  GlobalAnswers::new(total_questions, total_unanswered, tags_total, tags_unanswered);
    return global_data;
}

pub async fn collect_team_data(questions: Vec<Item>, site: &String, members: &Vec<u32>, options: &Options) -> Answers  {
    let mut answers_by_member = Vec::new();
    let mut team_answered =  TeamAnswers::new(0,0,0);
    for question in &questions {
        if question.is_answered.unwrap() {
            let answers: APIResponse = api::get_answers(question.question_id, site).await;
            team_answered = team_answered.question_answered(parse_answers(answers, &mut answers_by_member, members, options));
        }
    }
    let answers: Answers = Answers::new(team_answered, answers_by_member);
    return answers
}

fn parse_answers(answers: APIResponse, answers_by_member_vec: &mut Vec<MemberAnswer>,team_members: &Vec<u32>, options: &Options) ->  TeamAnswers {
    let mut team_answered =  TeamAnswers::new(0,0,0);
    for answer in &answers.items {
        if team_members.contains(&answer.owner.user_id)  {
            if options.individual {
                add_member_response(answers_by_member_vec, &answer.owner.user_id);
            }
            let aux = TeamAnswers::new(
                1, answer.score, answer.is_accepted.unwrap_or(false) as u32
            );
            team_answered = team_answered.question_answered(aux);
        }
    }
    
    return team_answered;
}

fn add_tags(tags_vec: &mut Vec<Tag>, question_tags: &Vec<String>) {
    for tag in question_tags {
        let exists = tags_vec.iter().find(|&x| x.name == tag.to_string()).is_some();
        if exists {
            let existing_tag_index = tags_vec.iter().position(|x| x.name == tag.to_string()).unwrap();
            let count = tags_vec[existing_tag_index].count;
            tags_vec[existing_tag_index] = Tag {name: tag.to_string(), count: count + 1}
        }
        else {
            tags_vec.push(Tag{name: tag.to_string(), count: 1});
        }
    }  
}

fn add_member_response(answers_by_member_vec: &mut Vec<MemberAnswer>, member_id: &u32) {
    let exists = answers_by_member_vec.iter().find(|&x| x.user_id == *member_id).is_some();
    if exists {
        let existing_member_index = answers_by_member_vec.iter().position(|x| x.user_id == *member_id).unwrap();
        let count = answers_by_member_vec[existing_member_index].count;
        answers_by_member_vec[existing_member_index] = MemberAnswer {user_id: *member_id, count: count + 1}
    }
    else {
        answers_by_member_vec.push(MemberAnswer{user_id: *member_id, count: 1});
    }
}