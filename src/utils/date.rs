use chrono::{Date, Local};
use chrono_english::Dialect;

pub type LocalDate = Date<Local>;

pub fn parse_date(date_str: &str) -> Option<LocalDate> {
    let date_str = date_str.replace('.', "/");
    if let Ok(datetime) = chrono_english::parse_date_string(&date_str, Local::now(), Dialect::Us) {
        Some(datetime.date())
    } else {
        None
    }
}
