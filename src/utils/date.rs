use chrono::{Date, Local};
use chrono_english::Dialect;

pub type LocalDate = Date<Local>;

pub fn parse_date(date_str: &str) -> Result<LocalDate, String> {
    let date_str = date_str.replace('.', "/");

    chrono_english::parse_date_string(&date_str, Local::now(), Dialect::Us)
        .map(|d| d.date())
        .map_err(|e| e.to_string())
}
