use chrono::{NaiveDate, NaiveDateTime};

pub fn get_epoch_ms(date: &str) -> i64 {
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