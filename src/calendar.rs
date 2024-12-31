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
fn gregorian_to_bengali_date(english_date: EnglishDate) -> Result<Date, DateError> {
    let (english_day, english_month, english_year) = english_date.get_date();

    // The Bengali year starts from 14th April
    // If the English date is before 14th April, the Bengali year is the English year - 594
    // Otherwise, the Bengali year is the English year - 593
    let bengali_year: u16 = if english_month < 4 || (english_month == 4 && english_day < 14) {
        match english_year.checked_sub(594) {
            Some(year) => year,
            None => {
                return Err(DateError::ArithmeticError);
            }
        }
    } else {
        match english_year.checked_sub(593) {
            Some(year) => year,
            None => {
                return Err(DateError::ArithmeticError);
            }
        }
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
        return Err(DateError::WrongDay);
    };

    let bengali_weekday =
        BengaliWeekDays::get_english_weekday(english_date.get_week_day().unwrap().as_str());

    match BengaliDate::create_date_with_weekday(
        bengali_day,
        bengali_weekday.unwrap(),
        BengaliMonths::get_month(bengali_month).unwrap(),
        bengali_year,
    ) {
        Ok(bengali_date) => Ok(Date::Bengali(bengali_date)),
        Err(err) => Err(err),
    }
}

fn bengali_to_gregorian_date(bengali_date: BengaliDate) -> Result<Date, DateError> {
    let (bengali_day, bengali_month, bengali_year) = bengali_date.get_date_number();

    let english_year: u16 = if (bengali_month == 9 && bengali_day >= 17)
        || (bengali_month == 12 && bengali_day <= 30)
        || bengali_month == 10
        || bengali_month == 11
    {
        match bengali_year.checked_add(594) {
            Some(year) => year,
            None => return Err(DateError::ArithmeticError),
        }
    } else {
        match bengali_year.checked_add(593) {
            Some(year) => year,
            None => return Err(DateError::ArithmeticError),
        }
    };

    let (english_date, english_month) = if bengali_month == 9 {
        if bengali_day >= 17 {
            (bengali_day - 16, 1)
        } else {
            (bengali_day + 15, 12)
        }
    } else if bengali_month == 10 {
        if bengali_day >= 18 {
            (bengali_day - 17, 2)
        } else {
            (bengali_day + 14, 1)
        }
    } else if bengali_month == 11 {
        if is_leap_year(english_year) {
            if bengali_day >= 17 {
                (bengali_day - 16, 3)
            } else {
                (bengali_day + 13, 2)
            }
        } else {
            if bengali_day >= 16 {
                (bengali_day - 15, 3)
            } else {
                (bengali_day + 13, 2)
            }
        }
    } else if bengali_month == 12 {
        if bengali_day >= 18 {
            (bengali_day - 17, 4)
        } else {
            (bengali_day + 14, 3)
        }
    } else if bengali_month == 1 {
        if bengali_day >= 18 {
            (bengali_day - 17, 5)
        } else {
            (bengali_day + 13, 4)
        }
    } else if bengali_month == 2 {
        if bengali_day >= 18 {
            (bengali_day - 17, 6)
        } else {
            (bengali_day + 14, 5)
        }
    } else if bengali_month == 3 {
        if bengali_day >= 17 {
            (bengali_day - 16, 7)
        } else {
            (bengali_day + 14, 6)
        }
    } else if bengali_month == 4 {
        if bengali_day >= 17 {
            (bengali_day - 16, 8)
        } else {
            (bengali_day + 15, 7)
        }
    } else if bengali_month == 5 {
        if bengali_day >= 17 {
            (bengali_day - 16, 9)
        } else {
            (bengali_day + 15, 8)
        }
    } else if bengali_month == 6 {
        if bengali_day >= 16 {
            (bengali_day - 15, 10)
        } else {
            (bengali_day + 15, 9)
        }
    } else if bengali_month == 7 {
        if bengali_day >= 16 {
            (bengali_day - 15, 11)
        } else {
            (bengali_day + 16, 10)
        }
    } else if bengali_month == 8 {
        if bengali_day >= 16 {
            (bengali_day - 15, 12)
        } else {
            (bengali_day + 15, 11)
        }
    } else {
        return Err(DateError::WrongDay);
    };

    match EnglishDate::create_date(
        english_date,
        EnglishMonths::get_month(english_month).unwrap(),
        english_year,
    ) {
        Ok(english_date) => Ok(Date::English(english_date)),
        Err(err) => Err(err),
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
                Ok(date) => match gregorian_to_bengali_date(date) {
                    Ok(bengali_date) => Ok(bengali_date),
                    Err(err) => Err(err),
                },
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
    match gregorian_to_bengali_date(english_date) {
        Ok(date) => Ok(date),
        Err(err) => Err(err),
    }
}

/// Get Gregorian date from Bengali date
/// # Arguments
/// * `bengali_date` - BengaliDate
/// # Returns
/// * `Result<Date, DateError>` - Gregorian date
/// # Example
/// ```
/// use ponjika::{calendar, BengaliDate, BengaliMonths};
/// let bengali_date = BengaliDate::create_date(3, BengaliMonths::Falgun, 1430);
/// match bengali_date {
///   Ok(bengali_date) => {
///     let gregorian_date = calendar::get_gregorian_date_from_bengali(bengali_date);
///     match gregorian_date {
///       Ok(date) => {
///         println!("Gregorian Date: {}", date.to_string());
///       }
///       Err(_) => {
///         eprintln!("Failed to convert to Gregorian date");
///       }
///     }
///   }
///   Err(_) => {
///     eprintln!("The date is not a valid Bengali date");
///   }
/// }
/// ```
/// # Note
/// * The function will return `DateError` if the conversion fails
pub fn get_gregorian_date_from_bengali(bengali_date: BengaliDate) -> Result<Date, DateError> {
    match bengali_to_gregorian_date(bengali_date) {
        Ok(date) => Ok(date),
        Err(err) => Err(err),
    }
}
