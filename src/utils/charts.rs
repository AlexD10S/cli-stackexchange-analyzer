use crate::primitives::Tag;
use piechart::{Chart, Color, Data};

const NUMBER_OF_HOT_TAGS: usize = 4;

pub fn display_chart_ratios(float_division_total: f32, float_division_total_team: f32) {
    let data = vec![
        Data {
            label: "Team Answers".into(),
            value: float_division_total_team,
            color: Some(Color::Blue.into()),
            fill: '•',
        },
        Data {
            label: "Unanswered".into(),
            value: float_division_total,
            color: Some(Color::Red.into()),
            fill: '▪',
        },
        Data {
            label: "Rest".into(),
            value: (100 as f32 - float_division_total_team - float_division_total),
            color: Some(Color::Yellow.into()),
            fill: '▴',
        },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}

pub fn display_chart_tags(sorted_list: &Vec<Tag>) {
    let colors = vec![
        Color::Blue.into(),
        Color::Red.into(),
        Color::Yellow.into(),
        Color::Green.into(),
    ];
    let mut data = vec![];


    let mut number_of_tags = NUMBER_OF_HOT_TAGS;
    if sorted_list.len() < number_of_tags {
        number_of_tags = sorted_list.len();
    }
    for n in 0..number_of_tags {
        data.push(Data {
            label: sorted_list[n].name.to_string(),
            value: sorted_list[n].count as f32,
            color: Some(colors[n % number_of_tags]),
            fill: '•',
        });
    }
    let rest_of_tags = sorted_list.len() - number_of_tags;
    data.push(Data {
        label: "Rest".to_string(),
        value: rest_of_tags as f32,
        color: Some(Color::White.into()),
        fill: '•',
    });

    Chart::new()
        .radius(7)
        .aspect_ratio(2)
        .legend(true)
        .draw(&data);
}
