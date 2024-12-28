use crate::date::Date;

/// Format Bengali date with weekday
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted Bengali date with weekday
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_bengali_date_with_weekday(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted Bengali date with weekday if the date is valid
/// * The formatted Bengali date with weekday will be in the format: `Weekday, Day Month Year`
pub fn format_bengali_date_with_weekday(date: Date) -> String {
    let bengali_date = date.get_bengali_date();

    match bengali_date {
        Some(bengali_date) => {
            let bengali_week_day = bengali_date.get_week_day();
            let bengali_month = bengali_date.get_month();
            let bengali_day = bengali_date.get_day();
            let bengali_year = bengali_date.get_year();

            format!(
                "{}, {} {} {}",
                bengali_week_day,
                bengali_day,
                bengali_month,
                bengali_year
            )
        }
        None => "Invalid Date".to_string(),        
    }
}

/// Format Bengali date
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted Bengali date
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_bengali_date(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted Bengali date if the date is valid
/// * The formatted Bengali date will be in the format: `Day Month Year`
pub fn format_bengali_date(date: Date) -> String {
    let bengali_date = date.get_bengali_date();

    match bengali_date {
        Some(bengali_date) => {
            let bengali_month = bengali_date.get_month();
            let bengali_day = bengali_date.get_day();
            let bengali_year = bengali_date.get_year();

            format!("{} {} {}", bengali_day, bengali_month, bengali_year)
        }
        None => "Invalid Date".to_string(),
    }
}

/// Format English date
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted English date
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_english_date(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted English date if the date is valid
/// * The formatted English date will be in the format: `Day Month Year`
pub fn format_english_date(date: Date) -> String {
    let english_date = date.get_english_date();

    match english_date {
        Some(english_date) => {
            let english_month = english_date.get_month();
            let english_day = english_date.get_day();
            let english_year = english_date.get_year();

            format!("{} {} {}", english_day, english_month, english_year)
        }
        None => "Invalid Date".to_string(),
    }
}

/// Format English date with weekday
/// # Arguments
/// * `date` - `Date`
/// # Returns
/// * `String` - Formatted English date with weekday
/// # Example
/// ```
/// use ponjika::*;
/// let date = get_bengali_date_from_gregorian(30, 3, 2012);
/// println!("{}", format_english_date_with_weekday(date));
/// ```
/// # Note
/// * The function will return `Invalid Date` if the date is invalid
/// * The function will return the formatted English date with weekday if the date is valid
/// * The formatted English date with weekday will be in the format: `Weekday, Day Month Year`
pub fn format_english_date_with_weekday(date: Date) -> String {
    let english_date = date.get_english_date();

    match english_date {
        Some(english_date) => {
            let english_week_day = english_date.get_week_day();
            let english_month = english_date.get_month();
            let english_day = english_date.get_day();
            let english_year = english_date.get_year();

            format!(
                "{}, {} {} {}",
                english_week_day,
                english_day,
                english_month,
                english_year
            )
        }
        None => "Invalid Date".to_string(),
    }
}