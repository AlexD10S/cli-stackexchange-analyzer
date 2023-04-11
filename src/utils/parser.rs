use crate::{
    api::dtos::APIResponse,
    primitives::{CliOptions, MemberAnswer, ResponseTime, Tag, TeamAnswersMetrics},
};

pub fn parse_answers(
    answers_of_question: APIResponse,
    answers_by_member: &mut Vec<MemberAnswer>,
    team_members: &Vec<u32>,
    response_time: &mut ResponseTime,
    options: &CliOptions,
) -> TeamAnswersMetrics {
    let mut team_answer_metrics = TeamAnswersMetrics::new(0, 0, 0);
    let mut set_time_set = true;

    for answer in &answers_of_question.items {
        // From API is already sorted by time of response
        if set_time_set {
            response_time.set_response_date(answer.creation_date);
            set_time_set = false;
        }
        if team_members.contains(&answer.owner.user_id.unwrap_or(0)) {
            team_answer_metrics = team_answer_metrics.add_question_answered_by_team(
                TeamAnswersMetrics::new(1, answer.score, answer.is_accepted.unwrap_or(false) as u32)
            );
            // Response time for the team responses
            response_time.set_response_date(answer.creation_date);
            response_time.set_team_answered(true);

            //If individual flag has been selected
            if options.individual {
                add_member_response(answers_by_member, &answer.owner.user_id.unwrap_or(0));
            }

        }
    }

    return team_answer_metrics;
}

pub fn add_member_response(answers_by_member_vec: &mut Vec<MemberAnswer>, member_id: &u32) {
    let exists = answers_by_member_vec
        .iter()
        .find(|&x| x.user_id == *member_id)
        .is_some();
    if exists {
        let existing_member_index = answers_by_member_vec
            .iter()
            .position(|x| x.user_id == *member_id)
            .unwrap();
        let count = answers_by_member_vec[existing_member_index].count;
        answers_by_member_vec[existing_member_index] = MemberAnswer {
            user_id: *member_id,
            count: count + 1,
        }
    } else {
        answers_by_member_vec.push(MemberAnswer {
            user_id: *member_id,
            count: 1,
        });
    }
}

pub fn parse_and_add_tags(tags_vec: &mut Vec<Tag>, question_tags: &Vec<String>) {
    for tag in question_tags {
        let exists = tags_vec
            .iter()
            .find(|&x| x.name == tag.to_string())
            .is_some();
        if exists {
            let existing_tag_index = tags_vec
                .iter()
                .position(|x| x.name == tag.to_string())
                .unwrap();
            let count = tags_vec[existing_tag_index].count;
            tags_vec[existing_tag_index] = Tag {
                name: tag.to_string(),
                count: count + 1,
            }
        } else {
            tags_vec.push(Tag {
                name: tag.to_string(),
                count: 1,
            });
        }
    }
}
