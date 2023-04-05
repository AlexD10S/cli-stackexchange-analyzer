use std::error::Error;
use csv;
use crate::{primitives::{GlobalData,Answers, CliOptions}};

pub fn export_data(date_start: &String, 
    date_end: &String, 
    options: &CliOptions, 
    global_data: &GlobalData, 
    answers: &Option<Answers>){
    //By default export it here
    let path = "./data_exported.csv";

    if let Err(e) = export_data_to_csv(&date_start, &date_end, &options, &global_data, &answers, &path) {
        eprintln!("Error exporting the data into a file: {}", e)
    }
    else{
        println!("Data exported in file {:?}", path)
    }
}

fn export_data_to_csv(
    date_start: &String, 
    date_end: &String, 
    options: &CliOptions, 
    global_data: &GlobalData, 
    answers: &Option<Answers>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(&[
        "Date Start",
        date_start,
        "Date End",
        date_end,
        "Site",
        &options.site,
    ])?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}
