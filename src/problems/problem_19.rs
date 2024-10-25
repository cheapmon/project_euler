use chrono::{Datelike, NaiveDate, Weekday};
use num::ToPrimitive;

pub(super) fn run() -> u64 {
    let start_date = NaiveDate::from_ymd_opt(1901, 01, 01).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2000, 12, 31).unwrap();

    start_date.iter_days()
        .take_while(|date| date <= &end_date)
        .filter(|date| date.weekday() == Weekday::Sun && date.day() == 1)
        .count()
        .to_u64()
        .unwrap()
}