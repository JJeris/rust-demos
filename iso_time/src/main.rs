// use chrono::{NaiveDateTime, DateTime, Utc, Datelike, Timelike};

// fn convert_to_iso8601(date_str: &str) -> Option<String> {
//     if let Ok(parsed_date_time) = chrono::NaiveDateTime::parse_from_str(date_str, "%d-%b-%Y %H:%M") {
//         let dt: DateTime<Utc> = TimeZone::from_utc_dwatetime(parsed_date_time, Utc);
//         Some(dt.to_rfc3339())
//     } else {
//         None
//     }
// }

// fn main() {
//     // Example date string
//     let date_str = "11-Sep-2017 13:19";
//     // let abc = 123

//     if let Some(iso8601_date) = convert_to_iso8601(date_str) {
//         println!("Date in ISO 8601 format: {}", iso8601_date);
//     } else {
//         println!("Failed to convert the date.");
//     }
// }

use chrono::NaiveDateTime;
// use chrono::format::strftime;
// use chrono::format::Parsed;
// use chrono::Timelike;

fn convert_to_iso(date_str: &str) -> Option<String> {
    // Define a format string that matches the input date format.
    let format = "%d-%b-%Y %H:%M";

    // Attempt to parse the input string using the specified format.
    if let Ok(parsed_date) = NaiveDateTime::parse_from_str(date_str, format) {
        // Convert the parsed date to the ISO format.
        let iso_date = parsed_date.format("%Y-%m-%dT%H:%M:%S").to_string();
        Some(iso_date)
    } else {
        None // Parsing failed, return None or handle the error as needed.
    }
}

fn main() {
    let date_str = "11-Sep-2017 13:20";
    if let Some(iso_date) = convert_to_iso(date_str) {
        println!("ISO Date: {}", iso_date);
    } else {
        println!("Failed to convert the date.");
    }
}