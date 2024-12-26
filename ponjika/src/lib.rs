mod calendar;
use chrono::prelude::*;

pub use calendar::{
    BengaliDate, BengaliDays, BengaliMonths, EnglishDate, EnglishDays, EnglishMonths,
};

fn gregorian_to_bengali_date(english_date: EnglishDate) -> BengaliDate {
    let bengali_year: u16 =
        if english_date.month < 4 || (english_date.month == 4 && english_date.day < 14) {
            english_date.year - 594
        } else {
            english_date.year - 593
        };

    let bengali_month = match english_date.month {
        4 if english_date.day >= 14 => (1, english_date.day - 13), // Boishakh starts April 14
        5 => (2, english_date.day + 17),                           // Joishtho
        6 => (3, english_date.day + 17),                           // Ashar
        7 => (4, english_date.day + 17),                           // Srabon
        8 => (5, english_date.day + 16),                           // Bhadro
        9 => (6, english_date.day + 15),                           // Ashwin
        10 => (7, english_date.day + 15),                          // Kartik
        11 => (8, english_date.day + 14),                          // Agrahayan
        12 => (9, english_date.day + 14),                          // Poush
        1 => (10, english_date.day + 15),                          // Magh
        2 => (11, english_date.day + 13),                          // Falgun
        3 => (12, english_date.day + 14),                          // Chaitra
        _ => (0, 0),
    };

    let (month_index, adjusted_day) = bengali_month;
    let bengali_day = if adjusted_day > 30 {
        adjusted_day - 30
    } else {
        adjusted_day
    };

    BengaliDate {
        day: bengali_day,
        week_day: BengaliDays::map_english(english_date.week_day),
        month: month_index,
        month_name: BengaliMonths::from_index(month_index),
        year: bengali_year,
    }
}

pub fn get_today_bengali_calendar() -> BengaliDate {
    let today = Local::now();
    let today_day = today.day() as u8;
    let today_month = today.month() as u8;
    let today_year = today.year() as u16;
    let today_week_day = match today.weekday() {
        Weekday::Mon => EnglishDays::Monday,
        Weekday::Tue => EnglishDays::Tuesday,
        Weekday::Wed => EnglishDays::Wednesday,
        Weekday::Thu => EnglishDays::Thursday,
        Weekday::Fri => EnglishDays::Friday,
        Weekday::Sat => EnglishDays::Saturday,
        Weekday::Sun => EnglishDays::Sunday,
    };

    println!(
        "Today is: {} ({}) {}, {}",
        today_day,
        today_week_day.as_str(),
        EnglishMonths::from_index(today_month).as_str(),
        today_year
    );

    let english_date = EnglishDate {
        day: today_day,
        month: today_month,
        month_name: EnglishMonths::from_index(today_month),
        year: today_year,
        week_day: today_week_day,
    };

    let bengali_date = gregorian_to_bengali_date(english_date);
    bengali_date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {}
}
