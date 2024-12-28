use chrono::{Datelike, TimeZone, Utc, Weekday};

use crate::days::{BengaliWeekDays, EnglishWeekDays};
use crate::months::{BengaliMonths, EnglishMonths, Month};

pub enum Date {
    English(EnglishDate),
    Bengali(BengaliDate),
    Invalid,
}

impl Date {
    pub fn get_english_date(self) -> Option<EnglishDate> {
        match self {
            Date::English(date) => Some(date),
            _ => None,
        }
    }

    pub fn get_bengali_date(self) -> Option<BengaliDate> {
        match self {
            Date::Bengali(date) => Some(date),
            _ => None,
        }
    }

    pub fn get_day(&self) -> String {
        match self {
            Date::English(date) => date.get_day(),
            Date::Bengali(date) => date.get_day(),
            Date::Invalid => "DateError: The day in the date was wrong".to_string(),
        }
    }

    pub fn get_week_day(&self) -> String {
        match self {
            Date::English(date) => date.get_week_day().to_string(),
            Date::Bengali(date) => date.get_week_day().to_string(),
            Date::Invalid => "DateError: The week day in the date was wrong".to_string(),
        }
    }

    pub fn get_month(&self) -> String {
        match self {
            Date::English(date) => date.get_month(),
            Date::Bengali(date) => date.get_month(),
            Date::Invalid => "DateError: The month in the date was wrong".to_string(),
        }
    }

    pub fn get_year(&self) -> String {
        match self {
            Date::English(date) => date.get_year(),
            Date::Bengali(date) => date.get_year(),
            Date::Invalid => "DateError: The year in the date was wrong".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct EnglishDate {
    day: u8,
    week_day: EnglishWeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl EnglishDate {
    fn is_valid_date(day: u8, month: u8, year: u16) -> bool {
        if day == 0 || month == 0 || year < 593 || year > 9999 {
            return false;
        }

        match year % 4 {
            0 => match month {
                1 => day <= 31,
                2 => day <= 29,
                3 => day <= 31,
                4 => day <= 30,
                5 => day <= 31,
                6 => day <= 30,
                7 => day <= 31,
                8 => day <= 31,
                9 => day <= 30,
                10 => day <= 31,
                11 => day <= 30,
                12 => day <= 31,
                _ => false,
            },
            _ => match month {
                1 => day <= 31,
                2 => day <= 28,
                3 => day <= 31,
                4 => day <= 30,
                5 => day <= 31,
                6 => day <= 30,
                7 => day <= 31,
                8 => day <= 31,
                9 => day <= 30,
                10 => day <= 31,
                11 => day <= 30,
                12 => day <= 31,
                _ => false,
            },
        }
    }

    pub fn create_date(day: u8, month: u8, year: u16) -> Date {
        if !Self::is_valid_date(day, month, year) {
            return Date::Invalid;
        }

        let week_day = match year {
            593..=9999 => {
                let date = Utc
                    .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
                    .unwrap();
                match date.weekday() {
                    Weekday::Mon => EnglishWeekDays::Monday,
                    Weekday::Tue => EnglishWeekDays::Tuesday,
                    Weekday::Wed => EnglishWeekDays::Wednesday,
                    Weekday::Thu => EnglishWeekDays::Thursday,
                    Weekday::Fri => EnglishWeekDays::Friday,
                    Weekday::Sat => EnglishWeekDays::Saturday,
                    Weekday::Sun => EnglishWeekDays::Sunday,
                }
            }
            _ => return Date::Invalid,
        };

        let month_name = match month {
            1 => Month::English(EnglishMonths::January),
            2 => Month::English(EnglishMonths::February),
            3 => Month::English(EnglishMonths::March),
            4 => Month::English(EnglishMonths::April),
            5 => Month::English(EnglishMonths::May),
            6 => Month::English(EnglishMonths::June),
            7 => Month::English(EnglishMonths::July),
            8 => Month::English(EnglishMonths::August),
            9 => Month::English(EnglishMonths::September),
            10 => Month::English(EnglishMonths::October),
            11 => Month::English(EnglishMonths::November),
            12 => Month::English(EnglishMonths::December),
            _ => return Date::Invalid,
        };

        Date::English(EnglishDate {
            day,
            week_day,
            month,
            month_name,
            year,
        })
    }

    pub fn get_day(&self) -> String {
        self.day.to_string()
    }

    pub fn get_day_number(&self) -> u8 {
        self.day
    }

    pub fn get_week_day(&self) -> String {
        self.week_day.get_name().to_string()
    }

    pub fn get_month(&self) -> String {
        self.month_name.get_name().to_string()
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

#[derive(Debug)]
pub struct BengaliDate {
    day: u8,
    week_day: BengaliWeekDays,
    month: u8,
    month_name: Month,
    year: u16,
}

impl BengaliDate {
    fn is_valid_date(day: u8, month: u8, year: u16) -> bool {
        if day == 0 || month == 0 || year < 1 || year > 8568 {
            return false;
        }

        match month % 4 {
            0 => match month {
                1 => day <= 31,
                2 => day <= 31,
                3 => day <= 31,
                4 => day <= 31,
                5 => day <= 31,
                6 => day <= 30,
                7 => day <= 30,
                8 => day <= 30,
                9 => day <= 30,
                10 => day <= 30,
                11 => day <= 30,
                12 => day <= 30,
                _ => false,
            },
            _ => match month {
                1 => day <= 31,
                2 => day <= 31,
                3 => day <= 31,
                4 => day <= 31,
                5 => day <= 31,
                6 => day <= 30,
                7 => day <= 30,
                8 => day <= 30,
                9 => day <= 30,
                10 => day <= 30,
                11 => day <= 31,
                12 => day <= 30,
                _ => false,
            },
        }
    }

    pub fn create_bengali_date(day: u8, week_day: BengaliWeekDays, month: u8, year: u16) -> Date {
        if !Self::is_valid_date(day, month, year) {
            return Date::Invalid;
        }

        Date::Bengali(BengaliDate {
            day,
            week_day,
            month,
            month_name: BengaliMonths::get_month(month),
            year,
        })
    }

    pub fn create_date(day: u8, month: u8, year: u16) -> Date {
        if !Self::is_valid_date(day, month, year) {
            return Date::Invalid;
        }

        let week_day = match year {
            0..=8568 => {
                let date = Utc
                    .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
                    .unwrap();
                match date.weekday() {
                    Weekday::Mon => BengaliWeekDays::Sombar,
                    Weekday::Tue => BengaliWeekDays::Mongolbar,
                    Weekday::Wed => BengaliWeekDays::Budhbar,
                    Weekday::Thu => BengaliWeekDays::Brihoshpotibar,
                    Weekday::Fri => BengaliWeekDays::Shukrobar,
                    Weekday::Sat => BengaliWeekDays::Shonibar,
                    Weekday::Sun => BengaliWeekDays::Robibar,
                }
            }
            _ => return Date::Invalid,
        };

        let month_name = match month {
            1 => Month::Bengali(BengaliMonths::Baishakh),
            2 => Month::Bengali(BengaliMonths::Jestha),
            3 => Month::Bengali(BengaliMonths::Ashad),
            4 => Month::Bengali(BengaliMonths::Shrawan),
            5 => Month::Bengali(BengaliMonths::Bhadra),
            6 => Month::Bengali(BengaliMonths::Ashwin),
            7 => Month::Bengali(BengaliMonths::Kartik),
            8 => Month::Bengali(BengaliMonths::Ogrohaeon),
            9 => Month::Bengali(BengaliMonths::Poush),
            10 => Month::Bengali(BengaliMonths::Magh),
            11 => Month::Bengali(BengaliMonths::Falgun),
            12 => Month::Bengali(BengaliMonths::Chaitra),
            _ => return Date::Invalid,
        };

        Date::Bengali(BengaliDate {
            day,
            week_day,
            month,
            month_name,
            year,
        })
    }

    pub fn get_day(&self) -> String {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];

        self.day
            .to_string()
            .chars()
            .map(|c| {
                let digit = c.to_digit(10).expect("DateError: Invalid digit");
                bengali_digits[digit as usize]
            })
            .collect()
    }

    pub fn get_year(&self) -> String {
        let bengali_digits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];

        self.year
            .to_string()
            .chars()
            .map(|c| {
                let digit = c.to_digit(10).expect("DateError: Invalid digit");
                bengali_digits[digit as usize]
            })
            .collect()
    }

    pub fn get_week_day(&self) -> String {
        self.week_day.get_name().to_string()
    }

    pub fn get_month(&self) -> String {
        self.month_name.get_name().to_string()
    }
}
