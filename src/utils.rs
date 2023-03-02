use chrono::{NaiveDate, NaiveDateTime};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Period {
    pub timestamp_start: i64,
    pub timestamp_end: i64,
}

pub fn get_period_in_ms(date_start: &str, date_end: &str) -> Period {
    let timestamp_start = get_epoch_ms(&date_start);
    let timestamp_end = get_epoch_ms(&date_end);
    return Period {timestamp_start, timestamp_end};
}

fn get_epoch_ms(date: &str) -> i64 {
    // Format of input is dd/mm/YYYY (can be changed)
    let naive_date = NaiveDate::parse_from_str(date, "%d/%m/%Y");
    match naive_date {
        Ok(content) => {  
            let naive_datetime: NaiveDateTime = content.and_hms_opt(0,0,0).unwrap();
            naive_datetime.timestamp()
        }
        Err(error) => { panic!("Error parsing the date {}, just exit here", error); }
    }
}

