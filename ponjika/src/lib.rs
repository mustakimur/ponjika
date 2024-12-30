pub mod calendar;
pub mod date;
pub mod days;
pub mod months;
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

    let bengali_weekday =
        BengaliWeekDays::map_to_english_weekday(english_date.get_week_day().unwrap().as_str());

    match BengaliDate::create_bengali_date(
        bengali_day,
        bengali_weekday.unwrap(),
        BengaliMonths::get_month(bengali_month).unwrap(),
        bengali_year,
    ) {
        Ok(bengali_date) => Date::Bengali(bengali_date),
        Err(_) => Date::Unknown,
    }
}

/// Get today's Bengali date
/// # Returns
/// * `Date` - Bengali date
/// # Example
/// ```
/// use ponjika::get_today_bengali_calendar;
/// let bengali_date = get_today_bengali_calendar();
/// println!("{:?}", bengali_date.get_bengali_date().unwrap());
/// ```
/// # Note
/// * The function will return `Date::Invalid` if the current date is invalid
/// * The function will return the Bengali date of the current date if the date is valid
pub fn get_today_bengali_calendar() -> Date {
    let today = Local::now();
    let today_day = today.day() as u8;
    let today_month = today.month() as u8;
    let today_year = today.year() as u16;

    match EnglishMonths::get_month(today_month) {
        Ok(month) => {
            let english_date = EnglishDate::create_date(
                today_day,
                month,
                today_year,
            );

            match english_date {
                Ok(date) => gregorian_to_bengali_date(date),
                Err(_) => Date::Unknown,
            }
        }
        Err(err) => {
            match err {
                _=> Date::Unknown,
            }
        },
    }
}

/// Get Bengali date from Gregorian date
/// # Arguments
/// * `day` - u8
/// * `month` - u8
/// * `year` - u16
/// # Returns
/// * `Date` - Bengali date
/// # Example
/// ```
/// ```
/// # Note
/// * The function will return `Date::Invalid` if the gregorian date is invalid
/// * The function will return the Bengali date if the date is valid
pub fn get_bengali_date_from_gregorian(english_date: EnglishDate) -> Date {
    let bengali_date = gregorian_to_bengali_date(english_date);
    bengali_date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {}
}
