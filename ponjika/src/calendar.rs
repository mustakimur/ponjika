//! # Calendar: Calendar related common functions
//! The module contains common functions related to calendar
//! The functions are used to format the date in Bengali and English

use chrono::{Datelike, Local};

use crate::{BengaliDate, BengaliMonths, BengaliWeekDays, Date, EnglishDate, EnglishMonths};

/// checks if the year is a leap year
/// # Arguments
/// * `year` - u16
/// # Returns
/// * `bool` - true if the year is a leap year
fn is_leap_year(year: u16) -> bool {
    year % 4 == 0
}

/// converts Gregorian date to Bengali date
/// # Arguments
/// * `english_date` - EnglishDate
/// # Returns
/// * `Date` - Bengali date
fn gregorian_to_bengali_date(english_date: EnglishDate) -> Date {
    let (english_day, english_month, english_year) = english_date.get_date();

    // The Bengali year starts from 14th April
    // If the English date is before 14th April, the Bengali year is the English year - 594
    // Otherwise, the Bengali year is the English year - 593
    let bengali_year: u16 = if english_month < 4 || (english_month == 4 && english_day < 14) {
        english_year - 594
    } else {
        english_year - 593
    };

    let (bengali_day, bengali_month) = if english_month == 1 {
        if english_day <= 14 {
            (english_day + 16, 9)
        } else {
            (english_day - 14, 10)
        }
    } else if english_month == 2 {
        if english_day <= 13 {
            (english_day + 17, 10)
        } else {
            (english_day - 13, 11)
        }
    } else if english_month == 3 {
        if english_day <= 14 {
            if is_leap_year(english_year) {
                (english_day + 16, 11)
            } else {
                (english_day + 15, 11)
            }
        } else {
            (english_day - 14, 12)
        }
    } else if english_month == 4 {
        if english_day <= 13 {
            (english_day + 17, 12)
        } else {
            (english_day - 13, 1)
        }
    } else if english_month == 5 {
        if english_day <= 14 {
            (english_day + 17, 1)
        } else {
            (english_day - 14, 2)
        }
    } else if english_month == 6 {
        if english_day <= 14 {
            (english_day + 17, 2)
        } else {
            (english_day - 14, 3)
        }
    } else if english_month == 7 {
        if english_day <= 15 {
            (english_day + 16, 3)
        } else {
            (english_day - 15, 4)
        }
    } else if english_month == 8 {
        if english_day <= 15 {
            (english_day + 16, 4)
        } else {
            (english_day - 15, 5)
        }
    } else if english_month == 9 {
        if english_day <= 15 {
            (english_day + 16, 5)
        } else {
            (english_day - 15, 6)
        }
    } else if english_month == 10 {
        if english_day <= 16 {
            (english_day + 15, 6)
        } else {
            (english_day - 16, 7)
        }
    } else if english_month == 11 {
        if english_day <= 15 {
            (english_day + 15, 7)
        } else {
            (english_day - 15, 8)
        }
    } else if english_month == 12 {
        if english_day <= 15 {
            (english_day + 15, 8)
        } else {
            (english_day - 15, 9)
        }
    } else {
        (0, 0)
    };

    let bengali_weekday =
        BengaliWeekDays::get_english_weekday(english_date.get_week_day().unwrap().as_str());

    match BengaliDate::create_date_with_weekday(
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
            let english_date = EnglishDate::create_date(today_day, month, today_year);

            match english_date {
                Ok(date) => gregorian_to_bengali_date(date),
                Err(_) => Date::Unknown,
            }
        }
        Err(err) => match err {
            _ => Date::Unknown,
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
