use crate::{primitives::Tag};

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
