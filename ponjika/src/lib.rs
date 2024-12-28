mod calendar;
mod date;
mod days;
mod months;
mod season;

pub use calendar::*;
use chrono::prelude::{Datelike, Local};
pub use date::*;
pub use days::*;
pub use months::*;

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0
}

fn gregorian_to_bengali_date(english_date: EnglishDate) -> Date {
    let english_year = english_date.get_year_number();
    let english_day = english_date.get_day_number();
    let english_month = english_date.get_month_number();

    let bengali_year: u16 = if english_month < 4 || (english_month == 4 && english_day < 14) {
        english_date.get_year_number() - 594
    } else {
        english_date.get_year_number() - 593
    };

    let mut bengali_day: u8 = 0;
    let mut bengali_month: u8 = 0;
    if english_month == 1 {
        if english_day <= 14 {
            bengali_day = english_day + 16;
            bengali_month = 9;
        } else {
            bengali_day = english_day - 14;
            bengali_month = 10;
        }
    } else if english_month == 2 {
        if english_day <= 13 {
            bengali_day = english_day + 17;
            bengali_month = 10;
        } else {
            bengali_day = english_day - 13;
            bengali_month = 11;
        }
    } else if english_month == 3 {
        if english_day <= 14 {
            if is_leap_year(english_year) {
                bengali_day = english_day + 16;
                bengali_month = 11;
            } else {
                bengali_day = english_day + 15;
                bengali_month = 11;
            }
        } else {
            bengali_day = english_day - 14;
            bengali_month = 12;
        }
    } else if english_month == 4 {
        if english_day <= 13 {
            bengali_day = english_day + 17;
            bengali_month = 12;
        } else {
            bengali_day = english_day - 13;
            bengali_month = 1;
        }
    } else if english_month == 5 {
        if english_day <= 14 {
            bengali_day = english_day + 17;
            bengali_month = 1;
        } else {
            bengali_day = english_day - 14;
            bengali_month = 2;
        }
    } else if english_month == 6 {
        if english_day <= 14 {
            bengali_day = english_day + 17;
            bengali_month = 2;
        } else {
            bengali_day = english_day - 14;
            bengali_month = 3;
        }
    } else if english_month == 7 {
        if english_day <= 15 {
            bengali_day = english_day + 16;
            bengali_month = 3;
        } else {
            bengali_day = english_day - 15;
            bengali_month = 4;
        }
    } else if english_month == 8 {
        if english_day <= 15 {
            bengali_day = english_day + 16;
            bengali_month = 4;
        } else {
            bengali_day = english_day - 15;
            bengali_month = 5;
        }
    } else if english_month == 9 {
        if english_day <= 15 {
            bengali_day = english_day + 16;
            bengali_month = 5;
        } else {
            bengali_day = english_day - 15;
            bengali_month = 6;
        }
    } else if english_month == 10 {
        if english_day <= 16 {
            bengali_day = english_day + 15;
            bengali_month = 6;
        } else {
            bengali_day = english_day - 16;
            bengali_month = 7;
        }
    } else if english_month == 11 {
        if english_day <= 15 {
            bengali_day = english_day + 15;
            bengali_month = 7;
        } else {
            bengali_day = english_day - 15;
            bengali_month = 8;
        }
    } else if english_month == 12 {
        if english_day <= 15 {
            bengali_day = english_day + 15;
            bengali_month = 8;
        } else {
            bengali_day = english_day - 15;
            bengali_month = 9;
        }
    }

    BengaliDate::create_bengali_date(
        bengali_day,
        BengaliWeekDays::map_english_name(english_date.get_week_day().as_str()),
        bengali_month,
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

pub fn get_bengali_date(day: u8, month: u8, year: u16) -> Date {
    let english_date = EnglishDate::create_date(day, month, year).get_english_date();

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
