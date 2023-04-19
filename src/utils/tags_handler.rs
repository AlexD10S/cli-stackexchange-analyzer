use crate::{primitives::Tag, api::dtos::Item};

pub fn filter_questions_by_tags(questions: &mut Vec<Item>, tag: &String) {
    questions.retain(|question| {
        let tags = question.tags.as_ref().unwrap();
        if tags.contains(tag) {
            return true;
        }
        false
    });
}

pub fn parse_and_add_tags(tags_vec: &mut Vec<Tag>, question_tags: &Vec<String>) {
    for tag in question_tags {
        let exists = tags_vec
            .iter()
            .any(|x| x.name == *tag);
        if exists {
            let existing_tag_index = tags_vec
                .iter()
                .position(|x| x.name == *tag)
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
