//! # Date: The module to represent the dates in English and Bengali
//! The `date` module is used to represent both English and Bengali dates.
//! The `Date` enum is used to represent both English and Bengali dates.
//! The `EnglishDate` and `BengaliDate` struct variants are the English and Bengali dates respectively.
//! The `DateError` enum is used to represent the error when the date is invalid.

use chrono::{Datelike, TimeZone, Utc, Weekday};

use crate::days::{BengaliWeekDays, EnglishWeekDays, WeekDayError, WeekDays};
use crate::months::{BengaliMonths, EnglishMonths, Month};
use crate::MonthError;

#[derive(Debug)]
pub enum DateError {
    UnknownDate,
    WrongWeekDay(WeekDayError),
    WrongMonth(MonthError),
    WrongDay,
    WrongYear,
    NumToCharError,
}

impl std::fmt::Display for DateError {
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
            DateError::NumToCharError => write!(f, "DateError: Failed to convert number to character"),
        }
    }
}

type DateResult = Result<(String, String, String, String), DateError>;

/// The enum `Date` is used to represent both English and Bengali dates.
/// The invalid variant is used when the date is invalid.
pub enum Date {
    English(EnglishDate),
    Bengali(BengaliDate),
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
    pub fn get_day(&self) -> DateResult {
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

/// The struct `EnglishDate` is used to represent the English date.
#[derive(Debug)]
pub struct EnglishDate {
    day: u8,
    week_day: WeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl EnglishDate {
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
    /// * The function will return `Date::Invalid` if the date is invalid
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
                let date = Utc
                    .with_ymd_and_hms(year as i32, month_index as u32, day as u32, 0, 0, 0)
                    .unwrap();
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

    pub fn get_day(&self) -> String {
        self.day.to_string()
    }

    pub fn get_day_number(&self) -> u8 {
        self.day
    }

    pub fn get_week_day(&self) -> Result<String, WeekDayError> {
        self.week_day.get_week_day()
    }

    pub fn get_month(&self) -> Result<String, MonthError> {
        self.month_name.get_month_name()
    }

    pub fn get_month_number(&self) -> u8 {
        self.month
    }

    pub fn get_year(&self) -> String {
        self.year.to_string()
    }

    pub fn get_year_number(&self) -> u16 {
        self.year
    }
}

/// The struct `BengaliDate` is used to represent the Bengali date.
#[derive(Debug)]
pub struct BengaliDate {
    day: u8,
    week_day: WeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl BengaliDate {
    fn is_valid_date(day: u8, month: u8, year: u16) -> Result<bool, DateError> {
        if day < 1 || day > 31 {
            return Err(DateError::WrongDay);
        }

        if year < 1 || year > 8568 {
            return Err(DateError::WrongYear);
        }

        match month % 4 {
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
    /// * The function will return `Date::Invalid` if the date is invalid
    pub fn create_bengali_date(
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
    /// * The function will return `Date::Invalid` if the date is invalid
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

        let week_day = match year {
            0..=8568 => {
                let date = Utc
                    .with_ymd_and_hms(year as i32, month_index as u32, day as u32, 0, 0, 0)
                    .unwrap();
                match date.weekday() {
                    Weekday::Mon => WeekDays::Bengali(BengaliWeekDays::Sombar),
                    Weekday::Tue => WeekDays::Bengali(BengaliWeekDays::Mongolbar),
                    Weekday::Wed => WeekDays::Bengali(BengaliWeekDays::Budhbar),
                    Weekday::Thu => WeekDays::Bengali(BengaliWeekDays::Brihoshpotibar),
                    Weekday::Fri => WeekDays::Bengali(BengaliWeekDays::Shukrobar),
                    Weekday::Sat => WeekDays::Bengali(BengaliWeekDays::Shonibar),
                    Weekday::Sun => WeekDays::Bengali(BengaliWeekDays::Robibar),
                }
            }
            _ => return Err(DateError::WrongWeekDay(WeekDayError::FailedDateTimes)),
        };

        Ok(BengaliDate {
            day,
            week_day,
            month: month_index,
            month_name: Month::Bengali(month),
            year,
        })
    }

    pub fn get_day(&self) -> Result<String, DateError> {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];
        self.day
            .to_string()
            .chars()
            .map(|c| {
                match c.to_digit(10) {
                    Some(digit) => Ok(bengali_digits[digit as usize]),
                    None => return Err(DateError::NumToCharError),
                }
            })
            .collect()
    }

    pub fn get_year(&self) -> Result<String, DateError> {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];

        self.year
            .to_string()
            .chars()
            .map(|c| {
                match c.to_digit(10) {
                    Some(digit) => Ok(bengali_digits[digit as usize]),
                    None => return Err(DateError::NumToCharError),
                }
            })
            .collect()
    }

    pub fn get_week_day(&self) -> Result<String, WeekDayError> {
        self.week_day.get_week_day()
    }

    pub fn get_month(&self) -> Result<String, MonthError> {
        self.month_name.get_month_name()
    }
}
