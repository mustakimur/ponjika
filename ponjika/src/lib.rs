mod calendar;
mod date;
mod days;
mod months;
mod season;

pub use calendar::*;
use chrono::prelude::{Datelike, Local, Weekday};
pub use date::*;
pub use days::*;
pub use months::*;

fn gregorian_to_bengali_date(english_date: EnglishDate) -> Date {
    let english_month = english_date.get_month_number();
    let bengali_year: u16 = if english_month < 4 || (english_month == 4 && english_month < 14) {
        english_date.get_year_number() - 594
    } else {
        english_date.get_year_number() - 593
    };

    let english_day = english_date.get_day_number();
    let bengali_month = match english_month {
        4 if english_day >= 14 => (1, english_day - 13), // Boishakh starts April 14
        5 => (2, english_day + 17),                      // Joishtho
        6 => (3, english_day + 17),                      // Ashar
        7 => (4, english_day + 17),                      // Srabon
        8 => (5, english_day + 16),                      // Bhadro
        9 => (6, english_day + 15),                      // Ashwin
        10 => (7, english_day + 15),                     // Kartik
        11 => (8, english_day + 14),                     // Agrahayan
        12 => (9, english_day + 14),                     // Poush
        1 => (10, english_day + 15),                     // Magh
        2 => (11, english_day + 13),                     // Falgun
        3 => (12, english_day + 14),                     // Chaitra
        _ => (0, 0),
    };

    let (bengali_month_index, adjusted_day) = bengali_month;
    let bengali_day = if adjusted_day > 30 {
        adjusted_day - 30
    } else {
        adjusted_day
    };

    BengaliDate::create_bengali_date(
        bengali_day,
        BengaliWeekDays::map_english_name(english_date.get_week_day().as_str()),
        bengali_month_index,
        bengali_year,
    )
}

pub fn get_today_bengali_calendar() -> Date {
    let today = Local::now();
    let today_day = today.day() as u8;
    let today_month = today.month() as u8;
    let today_year = today.year() as u16;

    let english_date =
        EnglishDate::create_date(today_day, today_month, today_year).get_english_date();

    match english_date {
        None => Date::Invalid,
        Some(date) => {
            let bengali_date = gregorian_to_bengali_date(date);
            bengali_date
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {}
}
