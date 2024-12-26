mod calendar;
use chrono::prelude::*;

pub use calendar::{EnglishMonths, BengaliMonths, EnglishDays, BengaliDays};

pub struct EnglishDate {
    day: u8,
    week_day: EnglishDays,
    month: EnglishMonths,
    year: u16,
}

pub struct BengaliDate {
    pub day: u8,
    pub week_day: BengaliDays,
    pub month: BengaliMonths,
    pub year: u16,
}

impl EnglishDate {
    pub fn new(day: u8, week_day: EnglishDays, month: EnglishMonths, year: u16) -> EnglishDate {
        EnglishDate { day, week_day, month, year }
    }
}

impl BengaliDate {
    pub fn new(day: u8, week_day: BengaliDays, month: BengaliMonths, year: u16) -> BengaliDate {
        BengaliDate { day, week_day, month, year }
    }
}

pub fn get_today_bengali_calendar() -> BengaliDate {
    let mut today_date: BengaliDate = BengaliDate::new(1, BengaliDays::Sombar, BengaliMonths::Baishakh, 1428);

    let today = Local::today();
    let today_day = today.day() as u8;
    let today_month = today.month() as u8;


    today_date.week_day = match today.weekday() {
        Weekday::Mon => BengaliDays::Sombar,
        Weekday::Tue => BengaliDays::Mongolbar,
        Weekday::Wed => BengaliDays::Budhbar,
        Weekday::Thu => BengaliDays::Brihoshpotibar,
        Weekday::Fri => BengaliDays::Shukrobar,
        Weekday::Sat => BengaliDays::Shonibar,
        Weekday::Sun => BengaliDays::Robibar,
    };

    println!("Today is: {} {} {}", today_day, today_month, today.year());

    today_date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {}
}
