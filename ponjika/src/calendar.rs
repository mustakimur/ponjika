//! # Calendar: Calendar related common functions
//! The module contains common functions related to calendar
//! The functions are used to format the date in Bengali and English

use chrono::{Datelike, Local};
use std::convert::TryInto;

use crate::{
    BengaliDate, BengaliMonths, BengaliWeekDays, Date, DateError, EnglishDate, EnglishMonths,
};

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

    // get the Bengali day and month
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
        unreachable!()
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
/// * `Result<Date, DateError>` - Bengali date
/// # Example
/// ```
/// use ponjika::calendar;
/// let today = calendar::get_today_bengali_date();
/// match today {
///   Ok(bengali_date) => {
///     println!("{}", bengali_date.to_string());
///   }
///   Err(_) => {
///     eprintln!("The date is not a valid greogrian date");
///   }
/// }
/// ```
/// # Note
/// * The function will return `DateError` if the system date is invalid
pub fn get_today_bengali_date() -> Result<Date, DateError> {
    let today = Local::now();

    let today_day: u8 = match today.day().try_into() {
        Ok(day) => day,
        Err(err) => {
            return Err(DateError::CastingError(err)); // or handle the error as needed
        }
    };

    let today_month: u8 = match today.month().try_into() {
        Ok(month) => month,
        Err(err) => {
            return Err(DateError::CastingError(err)); // or handle the error as needed
        }
    };

    let today_year: u16 = match today.year().try_into() {
        Ok(year) => year,
        Err(err) => {
            return Err(DateError::CastingError(err)); // or handle the error as needed
        }
    };

    match EnglishMonths::get_month(today_month) {
        Ok(month) => {
            let english_date = EnglishDate::create_date(today_day, month, today_year);

            match english_date {
                Ok(date) => Ok(gregorian_to_bengali_date(date)),
                Err(err) => return Err(err),
            }
        }
        Err(err) => match err {
            _ => return Err(DateError::WrongMonth(err)),
        },
    }
}

/// Get Bengali date from Gregorian date
/// # Arguments
/// * `english_date` - EnglishDate
/// # Returns
/// * `Result<Date, DateError>` - Bengali date
/// # Example
/// ```
/// use ponjika::{calendar, EnglishDate, EnglishMonths};
/// let date = EnglishDate::create_date(10, EnglishMonths::October, 2010);
/// match date {
///  Ok(english_date) => {
///   let bengali_date = calendar::get_bengali_date_from_gregorian(english_date);
///   match bengali_date {
///    Ok(date) => {
///     println!("{}", date.to_string());
///    }
///    Err(_) => {
///     eprintln!("Failed to convert to Bengali date");
///    }
///   }
///  }
///  Err(_) => {
///   eprintln!("The date is not a valid greogrian date");
///  }
/// }
/// ```
/// # Note
/// * The function will return `DateError` if the conversion fails
pub fn get_bengali_date_from_gregorian(english_date: EnglishDate) -> Result<Date, DateError> {
    let bengali_date = gregorian_to_bengali_date(english_date);
    Ok(bengali_date)
}
