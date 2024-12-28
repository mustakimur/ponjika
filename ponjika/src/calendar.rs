//! # Calendar: Calendar related common functions
//! The module contains common functions related to calendar
//! The functions are used to format the date in Bengali and English

use crate::date::Date;

/// Format date with weekday
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted date with weekday
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_date_with_weekday(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted date with weekday if the date is valid
/// * The formatted date with weekday will be in the format: `Weekday, Day Month Year`
pub fn format_date_with_weekday(date: Date) -> String {
    match date {
        Date::Bengali(bengali_date) => {
            let bengali_week_day = bengali_date.get_week_day();
            let bengali_month = bengali_date.get_month();
            let bengali_day = bengali_date.get_day();
            let bengali_year = bengali_date.get_year();

            format!(
                "{}, {} {} {}",
                bengali_week_day, bengali_day, bengali_month, bengali_year
            )
        }
        Date::English(english_date) => {
            let english_week_day = english_date.get_week_day();
            let english_month = english_date.get_month();
            let english_day = english_date.get_day();
            let english_year = english_date.get_year();

            format!(
                "{}, {} {} {}",
                english_week_day, english_day, english_month, english_year
            )
        }
        Date::Invalid => "Invalid Date".to_string(),
    }
}

/// Format date without weekday
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted date
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_date(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted date if the date is valid
/// * The formatted date will be in the format: `Day Month Year`
pub fn format_date(date: Date) -> String {
    match date {
        Date::Bengali(bengali_date) => {
            let bengali_month = bengali_date.get_month();
            let bengali_day = bengali_date.get_day();
            let bengali_year = bengali_date.get_year();

            format!("{} {} {}", bengali_day, bengali_month, bengali_year)
        }
        Date::English(english_date) => {
            let english_month = english_date.get_month();
            let english_day = english_date.get_day();
            let english_year = english_date.get_year();

            format!("{} {} {}", english_day, english_month, english_year)
        }
        Date::Invalid => "Invalid Date".to_string(),
    }
}
