//! # Date: The module to represent the dates in English and Bengali
//! The `date` module is used to represent both English and Bengali dates.
//! The `Date` enum is used to represent both English and Bengali dates.
//! The `EnglishDate` and `BengaliDate` struct variants are the English and Bengali dates respectively.
//! The `DateError` enum is used to represent the error when the date is invalid.

use std::fmt::{self};
use std::num::TryFromIntError;

use chrono::{Datelike, TimeZone, Utc, Weekday};

use crate::days::{BengaliWeekDays, EnglishWeekDays, WeekDayError, WeekDays};
use crate::months::{BengaliMonths, EnglishMonths, Month};
use crate::MonthError;

/// # `DateError`: The error enum for the dates.
/// The enum variant is the error message.
#[derive(Debug)]
pub enum DateError {
    /// The UnknownDate variant is used when the date is invalid.
    UnknownDate,
    /// The WrongWeekDay variant is used when the week day is invalid.
    WrongWeekDay(WeekDayError),
    /// The WrongMonth variant is used when the month is invalid.
    WrongMonth(MonthError),
    /// The WrongDay variant is used when the day is invalid.
    WrongDay,
    /// The WrongYear variant is used when the year is invalid.
    WrongYear,
    /// The NumToCharError variant is used when the number to character conversion failed.
    NumToCharError,
    /// The CastingError variant is used when the casting failed.
    CastingError(TryFromIntError),
    /// The ArithmeticError variant is used when the arithmetic operation failed.
    ArithmeticError,
}

impl std::fmt::Display for DateError {
    /// Display the error message
    /// # Returns
    /// * `fmt::Result` - The error message
    /// # Example
    /// ```
    /// use ponjika::date::DateError;
    /// let error = DateError::UnknownDate;
    /// println!("{}", error);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DateError::UnknownDate => write!(f, "DateError: Unknown date"),
            DateError::WrongWeekDay(err) => {
                write!(f, "DateError: The week day in the date was wrong: {}", err)
            }
            DateError::WrongMonth(err) => {
                write!(f, "DateError: The month in the date was wrong: {}", err)
            }
            DateError::WrongDay => write!(f, "DateError: The day in the date was wrong"),
            DateError::WrongYear => write!(f, "DateError: The year in the date was wrong"),
            DateError::NumToCharError => {
                write!(f, "DateError: Failed to convert number to character")
            }
            DateError::CastingError(err) => {
                write!(f, "DateError: Failed to cast the number: {}", err)
            }
            DateError::ArithmeticError => write!(f, "DateError: Failed to perform arithmetic operation"),
        }
    }
}

// Use the Result type to handle the error for the dates
type DateResult = Result<(String, String, String, String), DateError>;

/// # `Date`: The enum for the dates.
/// The enum variants are the English and Bengali dates.
pub enum Date {
    /// The English variant is used to represent the English dates.
    English(EnglishDate),
    /// The Bengali variant is used to represent the Bengali dates.
    Bengali(BengaliDate),
    /// The Unknown variant is used when the date is invalid.
    Unknown,
}

impl Date {
    /// Get the English date from the selected date
    /// # Returns
    /// * `Option<EnglishDate>` - The English date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the English date
    /// * The function will return `None` if the date is not English
    pub fn get_english_date(self) -> Option<EnglishDate> {
        match self {
            Date::English(date) => Some(date),
            _ => None,
        }
    }

    /// Get the Bengali date from the selected date
    /// # Returns
    /// * `Option<BengaliDate>` - The Bengali date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the Bengali date
    /// * The function will return `None` if the date is not Bengali
    pub fn get_bengali_date(self) -> Option<BengaliDate> {
        match self {
            Date::Bengali(date) => Some(date),
            _ => None,
        }
    }

    /// Get the day of the selected date
    /// # Returns
    /// * `String` - The day of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the day of the date
    /// * The function will return "DateError: The day in the date was wrong" if the day is invalid
    pub fn get_date(&self) -> DateResult {
        match self {
            Date::English(date) => Ok((
                date.get_day(),
                match date.get_week_day() {
                    Ok(week_day) => week_day,
                    Err(err) => return Err(DateError::WrongWeekDay(err)),
                },
                match date.get_month() {
                    Ok(month) => month,
                    Err(err) => return Err(DateError::WrongMonth(err)),
                },
                date.get_year(),
            )),
            Date::Bengali(date) => Ok((
                match date.get_day() {
                    Ok(day) => day,
                    Err(err) => return Err(err),
                },
                match date.get_week_day() {
                    Ok(week_day) => week_day,
                    Err(err) => return Err(DateError::WrongWeekDay(err)),
                },
                match date.get_month() {
                    Ok(month) => month,
                    Err(err) => return Err(DateError::WrongMonth(err)),
                },
                match date.get_year() {
                    Ok(year) => year,
                    Err(err) => return Err(err),
                },
            )),
            Date::Unknown => Err(DateError::UnknownDate),
        }
    }
}

impl fmt::Display for Date {
    /// Display the date
    /// # Returns
    /// * `fmt::Result` - The date
    /// # Example
    /// ```
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Date::English(date) => write!(
                f,
                "{1}, {0} {2} {3}",
                date.get_day(),
                date.get_week_day().unwrap(),
                date.get_month().unwrap(),
                date.get_year()
            ),
            Date::Bengali(date) => write!(
                f,
                "{1}, {0} {2} {3}",
                date.get_day().unwrap(),
                date.get_week_day().unwrap(),
                date.get_month().unwrap(),
                date.get_year().unwrap()
            ),
            Date::Unknown => write!(f, "Unknown date"),
        }
    }
}

/// # `EnglishDate`: The struct for the English date.
/// The struct is used to represent the English date.
#[derive(Debug)]
pub struct EnglishDate {
    day: u8,
    week_day: WeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl EnglishDate {
    /// Check if the date is valid
    /// # Arguments
    /// * `day` - u8
    /// * `month` - u8
    /// * `year` - u16
    /// # Returns
    /// * `Result<bool>` - The result of the date validation
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return `Ok(true)` if the date is valid
    /// * The function will return `Err(DateError)` if the date is invalid
    fn is_valid_date(day: u8, month: u8, year: u16) -> Result<bool, DateError> {
        if day < 1 || day > 31 {
            return Err(DateError::WrongDay);
        }

        if year < 593 || year > 9999 {
            return Err(DateError::WrongYear);
        }

        match year % 4 {
            0 => match month {
                1 => Ok(day <= 31),
                2 => Ok(day <= 29),
                3 => Ok(day <= 31),
                4 => Ok(day <= 30),
                5 => Ok(day <= 31),
                6 => Ok(day <= 30),
                7 => Ok(day <= 31),
                8 => Ok(day <= 31),
                9 => Ok(day <= 30),
                10 => Ok(day <= 31),
                11 => Ok(day <= 30),
                12 => Ok(day <= 31),
                _ => Err(DateError::WrongMonth(MonthError::WrongRange)),
            },
            _ => match month {
                1 => Ok(day <= 31),
                2 => Ok(day <= 28),
                3 => Ok(day <= 31),
                4 => Ok(day <= 30),
                5 => Ok(day <= 31),
                6 => Ok(day <= 30),
                7 => Ok(day <= 31),
                8 => Ok(day <= 31),
                9 => Ok(day <= 30),
                10 => Ok(day <= 31),
                11 => Ok(day <= 30),
                12 => Ok(day <= 31),
                _ => Err(DateError::WrongMonth(MonthError::WrongRange)),
            },
        }
    }

    /// Create an English date
    /// # Arguments
    /// * `day` - u8
    /// * `month` - u8
    /// * `year` - u16
    /// # Returns
    /// * `Date` - The English date
    /// # Example
    /// ```
    /// use ponjika::date::EnglishDate;
    /// use ponjika::months::EnglishMonths;
    /// let date = EnglishDate::create_date(1, EnglishMonths::January, 2021);
    /// ```
    /// # Note
    /// * The function will return the English date
    /// * The function will return `DateError` if there is an error
    pub fn create_date(day: u8, month: EnglishMonths, year: u16) -> Result<Self, DateError> {
        let month_index = month.map_to_index();
        match Self::is_valid_date(day, month_index, year) {
            Ok(valid) => {
                if !valid {
                    return Err(DateError::UnknownDate);
                }
            }
            Err(err) => return Err(err),
        }

        let week_day = match year {
            593..=9999 => {
                let date = match Utc.with_ymd_and_hms(
                    year as i32,
                    month_index as u32,
                    day as u32,
                    0,
                    0,
                    0,
                ) {
                    chrono::LocalResult::Single(date) => date,
                    _ => return Err(DateError::UnknownDate),
                };

                match date.weekday() {
                    Weekday::Mon => WeekDays::English(EnglishWeekDays::Monday),
                    Weekday::Tue => WeekDays::English(EnglishWeekDays::Tuesday),
                    Weekday::Wed => WeekDays::English(EnglishWeekDays::Wednesday),
                    Weekday::Thu => WeekDays::English(EnglishWeekDays::Thursday),
                    Weekday::Fri => WeekDays::English(EnglishWeekDays::Friday),
                    Weekday::Sat => WeekDays::English(EnglishWeekDays::Saturday),
                    Weekday::Sun => WeekDays::English(EnglishWeekDays::Sunday),
                }
            }
            _ => return Err(DateError::WrongWeekDay(WeekDayError::FailedDateTimes)),
        };

        Ok(EnglishDate {
            day,
            week_day,
            month: month_index,
            month_name: Month::English(month),
            year,
        })
    }

    /// Get the date in numbers of the selected date
    /// # Returns
    /// * `(u8, u8, u16)` - The day, month, and year of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the day, month, and year of the date
    pub fn get_date(&self) -> (u8, u8, u16) {
        (self.day, self.month, self.year)
    }

    /// Get the day of the selected date
    /// # Returns
    /// * `String` - The day of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the day of the date
    pub fn get_day(&self) -> String {
        self.day.to_string()
    }

    /// Get the week day of the selected date
    /// # Returns
    /// * `String` - The week day of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the week day of the date
    /// * The function will return `WeekDayError` if there is an error
    pub fn get_week_day(&self) -> Result<String, WeekDayError> {
        self.week_day.get_week_day()
    }

    /// Get the month of the selected date
    /// # Returns
    /// * `String` - The month of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the month of the date
    /// * The function will return `MonthError` if there is an error
    pub fn get_month(&self) -> Result<String, MonthError> {
        self.month_name.get_month_name()
    }

    /// Get the year of the selected date
    /// # Returns
    /// * `String` - The year of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the year of the date
    pub fn get_year(&self) -> String {
        self.year.to_string()
    }
}

impl fmt::Display for EnglishDate {
    /// Display the date
    /// # Returns
    /// * `fmt::Result` - The date
    /// # Example
    /// ```
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{1}, {0} {2} {3}",
            self.get_day(),
            self.get_week_day().unwrap(),
            self.get_month().unwrap(),
            self.get_year()
        )
    }
}

/// # `BengaliDate`: The struct for the Bengali date.
/// The struct is used to represent the Bengali date.
#[derive(Debug)]
pub struct BengaliDate {
    day: u8,
    week_day: WeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl BengaliDate {
    /// Check if the date is valid
    /// # Arguments
    /// * `day` - u8
    /// * `month` - u8
    /// * `year` - u16
    /// # Returns
    /// * `Result<bool>` - The result of the date validation
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return `Ok(true)` if the date is valid
    /// * The function will return `Err(DateError)` if the date is invalid
    fn is_valid_date(day: u8, month: u8, year: u16) -> Result<bool, DateError> {
        if day < 1 || day > 31 {
            return Err(DateError::WrongDay);
        }

        if year < 1 || year > 8568 {
            return Err(DateError::WrongYear);
        }

        match month % 4 {
            // leap year
            0 => match month {
                1 => Ok(day <= 31),
                2 => Ok(day <= 31),
                3 => Ok(day <= 31),
                4 => Ok(day <= 31),
                5 => Ok(day <= 31),
                6 => Ok(day <= 30),
                7 => Ok(day <= 30),
                8 => Ok(day <= 30),
                9 => Ok(day <= 30),
                10 => Ok(day <= 30),
                11 => Ok(day <= 30),
                12 => Ok(day <= 30),
                _ => Err(DateError::WrongMonth(MonthError::WrongRange)),
            },
            _ => match month {
                1 => Ok(day <= 31),
                2 => Ok(day <= 31),
                3 => Ok(day <= 31),
                4 => Ok(day <= 31),
                5 => Ok(day <= 31),
                6 => Ok(day <= 30),
                7 => Ok(day <= 30),
                8 => Ok(day <= 30),
                9 => Ok(day <= 30),
                10 => Ok(day <= 30),
                11 => Ok(day <= 31),
                12 => Ok(day <= 30),
                _ => Err(DateError::WrongMonth(MonthError::WrongRange)),
            },
        }
    }

    /// Create a Bengali date
    /// # Arguments
    /// * `day` - u8
    /// * `week_day` - BengaliWeekDays
    /// * `month` - u8
    /// * `year` - u16
    /// # Returns
    /// * `Date` - The Bengali date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the Bengali date
    /// * The function will return `DateError` if the date is invalid
    pub fn create_date_with_weekday(
        day: u8,
        week_day: BengaliWeekDays,
        month: BengaliMonths,
        year: u16,
    ) -> Result<Self, DateError> {
        let month_index = month.map_to_index();
        match Self::is_valid_date(day, month_index, year) {
            Ok(valid) => {
                if !valid {
                    return Err(DateError::UnknownDate);
                }
            }
            Err(err) => return Err(err),
        }

        Ok(BengaliDate {
            day,
            week_day: WeekDays::Bengali(week_day),
            month: month_index,
            month_name: Month::Bengali(BengaliMonths::get_month(month_index).unwrap()),
            year,
        })
    }

    /// Create a Bengali date
    /// # Arguments
    /// * `day` - u8
    /// * `month` - u8
    /// * `year` - u16
    /// # Returns
    /// * `Date` - The Bengali date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the Bengali date
    /// * The function will return `DateError` if the date is invalid
    pub fn create_date(day: u8, month: BengaliMonths, year: u16) -> Result<Self, DateError> {
        let month_index = month.map_to_index();
        match Self::is_valid_date(day, month_index, year) {
            Ok(valid) => {
                if !valid {
                    return Err(DateError::UnknownDate);
                }
            }
            Err(_) => return Err(DateError::UnknownDate),
        }

        Ok(BengaliDate {
            day,
            week_day: WeekDays::Bengali(BengaliWeekDays::UnImplemented),
            month: month_index,
            month_name: Month::Bengali(month),
            year,
        })
    }

    pub fn get_date_number(&self) -> (u8, u8, u16) {
        (self.day, self.month, self.year)
    }

    /// Get the day of the selected date
    /// # Returns
    /// * `Result<String>` - The day of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the day of the date
    /// * The function will return "DateError: Failed to convert number to character" if the day is invalid
    pub fn get_day(&self) -> Result<String, DateError> {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];
        self.day
            .to_string()
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(digit) => Ok(bengali_digits[digit as usize]),
                None => return Err(DateError::NumToCharError),
            })
            .collect()
    }

    /// Get the week day of the selected date
    /// # Returns
    /// * `Result<String>` - The week day of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the week day of the date
    /// * The function will return "WeekDayError: Unknown week days" if the week day is invalid
    pub fn get_week_day(&self) -> Result<String, WeekDayError> {
        self.week_day.get_week_day()
    }

    /// Get the month of the selected date
    /// # Returns
    /// * `Result<String>` - The month of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the month of the date
    /// * The function will return "MonthError: Unknown month" if the month is invalid
    pub fn get_month(&self) -> Result<String, MonthError> {
        self.month_name.get_month_name()
    }

    /// Get the year of the selected date
    /// # Returns
    /// * `Result<String>` - The year of the date
    /// # Example
    /// ```
    /// ```
    /// # Note
    /// * The function will return the year of the date
    /// * The function will return "DateError: Failed to convert number to character" if the year is invalid
    pub fn get_year(&self) -> Result<String, DateError> {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];

        self.year
            .to_string()
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(digit) => Ok(bengali_digits[digit as usize]),
                None => return Err(DateError::NumToCharError),
            })
            .collect()
    }
}

impl fmt::Display for BengaliDate {
    /// Display the date
    /// # Returns
    /// * `fmt::Result` - The date
    /// # Example
    /// ```
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{1}, {0} {2} {3}",
            self.get_day().unwrap(),
            self.get_week_day().unwrap(),
            self.get_month().unwrap(),
            self.get_year().unwrap()
        )
    }
}
