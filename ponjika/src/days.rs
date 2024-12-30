//! # Days: The week day names in English and Bengali
//! The `days` module provides the week day names in English and Bengali.
//! The `WeekDays` enum is used to represent both English and Bengali week days.
//! The `EnglishWeekDays` and `BengaliWeekDays` enum variants are the English and Bengali week days respectively.
//! The `WeekDayError` enum is used to represent the error when the week day is invalid.

use std::fmt;

/// The enum `WeekDayError` is used to represent the error when the week day is invalid.
/// The enum variant is the error message.
/// The UnknownWeekDays variant is used when the week day is neither English nor Bengali.
#[derive(Debug)]
pub enum WeekDayError {
    UnknownWeekDays,
    FailedDateTimes,
}

impl fmt::Display for WeekDayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeekDayError::UnknownWeekDays => write!(f, "WeekDayError: Unknown week days"),
            WeekDayError::FailedDateTimes => write!(f, "WeekDayError: Failed to get date and time"),
        }
    }
}

type Result<T> = std::result::Result<T, WeekDayError>;

/// The enum `WeekDays` is used to represent both English and Bengali week days.
/// The enum variants are the English and Bengali week days.
/// The WeekDayError variant is used when the week day is invalid.
#[derive(Debug)]
pub enum WeekDays {
    English(EnglishWeekDays),
    Bengali(BengaliWeekDays),
    Unknown,
}

impl WeekDays {
    /// Get the week day name of the selected week day
    /// # Returns
    /// * `Result<String>` - The name of the week day
    /// # Example
    /// ```
    /// use ponjika::days::{WeekDays, EnglishWeekDays, BengaliWeekDays};
    /// let week_day = WeekDays::English(EnglishWeekDays::Sunday);
    /// assert_eq!(week_day.get_week_day().unwrap(), "Sunday");
    /// let week_day = WeekDays::Bengali(BengaliWeekDays::Robibar);
    /// assert_eq!(week_day.get_week_day().unwrap(), "রবিবার");
    /// ```
    /// # Note
    /// * The function will return the name of the week day
    /// * The function will return "WeekDayError: Unknown week days" if the week day is invalid
    pub fn get_week_day(&self) -> Result<String> {
        match self {
            WeekDays::English(day) => Ok(day.to_string()),
            WeekDays::Bengali(day) => Ok(day.to_string()),
            WeekDays::Unknown => Err(WeekDayError::UnknownWeekDays),
        }
    }
}

/// The enum `EnglishWeekDays` is used to represent the English week days.
/// The enum variants are the English week days.
#[derive(Debug)]
pub enum EnglishWeekDays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl EnglishWeekDays {
    /// Map the English week day to the Bengali week day
    /// # Returns
    /// * `BengaliWeekDays` - The Bengali week day
    /// # Example
    /// ```
    /// use ponjika::days::EnglishWeekDays;
    /// let week_day = EnglishWeekDays::Sunday;
    /// println!("{:?}", week_day.map_to_bengali());
    /// ```
    /// # Note
    /// * The function will return the Bengali week day
    pub fn map_to_bengali(&self) -> BengaliWeekDays {
        match self {
            EnglishWeekDays::Sunday => BengaliWeekDays::Robibar,
            EnglishWeekDays::Monday => BengaliWeekDays::Sombar,
            EnglishWeekDays::Tuesday => BengaliWeekDays::Mongolbar,
            EnglishWeekDays::Wednesday => BengaliWeekDays::Budhbar,
            EnglishWeekDays::Thursday => BengaliWeekDays::Brihoshpotibar,
            EnglishWeekDays::Friday => BengaliWeekDays::Shukrobar,
            EnglishWeekDays::Saturday => BengaliWeekDays::Shonibar,
        }
    }

    /// Map the Bengali week day name to the English week day
    /// # Arguments
    /// * `week_day` - &str
    /// # Returns
    /// * `EnglishWeekDays` - The English week day
    /// # Example
    /// ```
    /// use ponjika::days::EnglishWeekDays;
    /// let week_day = EnglishWeekDays::map_to_english_weekday("রবিবার");
    /// match week_day {
    ///   Some(EnglishWeekDays::Sunday) => println!("Sunday"),
    ///   Some(_) => println!("Other days"),
    ///   None => println!("Invalid week day"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the English week day
    /// * The function will return None if the week day is invalid
    pub fn map_to_english_weekday(week_day: &str) -> Option<Self> {
        match week_day {
            "রবিবার" => Some(EnglishWeekDays::Sunday),
            "সোমবার" => Some(EnglishWeekDays::Monday),
            "মঙ্গলবার" => Some(EnglishWeekDays::Tuesday),
            "বুধবার" => Some(EnglishWeekDays::Wednesday),
            "বৃহস্পতিবার" => Some(EnglishWeekDays::Thursday),
            "শুক্রবার" => Some(EnglishWeekDays::Friday),
            "শনিবার" => Some(EnglishWeekDays::Saturday),
            _ => None,
        }
    }
}

impl fmt::Display for EnglishWeekDays {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day_str = match self {
            EnglishWeekDays::Sunday => "Sunday",
            EnglishWeekDays::Monday => "Monday",
            EnglishWeekDays::Tuesday => "Tuesday",
            EnglishWeekDays::Wednesday => "Wednesday",
            EnglishWeekDays::Thursday => "Thursday",
            EnglishWeekDays::Friday => "Friday",
            EnglishWeekDays::Saturday => "Saturday",
        };
        write!(f, "{}", day_str)
    }
}

/// The enum `BengaliWeekDays` is used to represent the Bengali week days.
/// The enum variants are the Bengali week days.
#[derive(Debug)]
pub enum BengaliWeekDays {
    Robibar,
    Sombar,
    Mongolbar,
    Budhbar,
    Brihoshpotibar,
    Shukrobar,
    Shonibar,
}

impl BengaliWeekDays {
    /// Map the Bengali week day to the English week day
    /// # Returns
    /// * `EnglishWeekDays` - The English week day
    /// # Example
    /// ```
    /// use ponjika::days::BengaliWeekDays;
    /// let week_day = BengaliWeekDays::Robibar;
    /// println!("{:?}", week_day.map_to_english());
    /// ```
    /// # Note
    /// * The function will return the English week day
    pub fn map_to_english(&self) -> EnglishWeekDays {
        match self {
            BengaliWeekDays::Robibar => EnglishWeekDays::Sunday,
            BengaliWeekDays::Sombar => EnglishWeekDays::Monday,
            BengaliWeekDays::Mongolbar => EnglishWeekDays::Tuesday,
            BengaliWeekDays::Budhbar => EnglishWeekDays::Wednesday,
            BengaliWeekDays::Brihoshpotibar => EnglishWeekDays::Thursday,
            BengaliWeekDays::Shukrobar => EnglishWeekDays::Friday,
            BengaliWeekDays::Shonibar => EnglishWeekDays::Saturday,
        }
    }

    /// Map the English week day name to the Bengali week day
    /// # Arguments
    /// * `week_day` - &str
    /// # Returns
    /// * `BengaliWeekDays` - The Bengali week day
    /// # Example
    /// ```
    /// use ponjika::days::BengaliWeekDays;
    /// let week_day = BengaliWeekDays::map_to_english_weekday("Sunday");
    /// match week_day {
    ///    Some(BengaliWeekDays::Robibar) => println!("রবিবার"),
    ///    Some(_) => println!("Other days"),
    ///    None => println!("Invalid week day"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the Bengali week day
    /// * The function will return None if the week day is invalid
    pub fn map_to_english_weekday(week_day: &str) -> Option<Self> {
        match week_day {
            "Sunday" => Some(BengaliWeekDays::Robibar),
            "Monday" => Some(BengaliWeekDays::Sombar),
            "Tuesday" => Some(BengaliWeekDays::Mongolbar),
            "Wednesday" => Some(BengaliWeekDays::Budhbar),
            "Thursday" => Some(BengaliWeekDays::Brihoshpotibar),
            "Friday" => Some(BengaliWeekDays::Shukrobar),
            "Saturday" => Some(BengaliWeekDays::Shonibar),
            _ => None,
        }
    }
}

impl fmt::Display for BengaliWeekDays {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day_str = match self {
            BengaliWeekDays::Robibar => "রবিবার",
            BengaliWeekDays::Sombar => "সোমবার",
            BengaliWeekDays::Mongolbar => "মঙ্গলবার",
            BengaliWeekDays::Budhbar => "বুধবার",
            BengaliWeekDays::Brihoshpotibar => "বৃহস্পতিবার",
            BengaliWeekDays::Shukrobar => "শুক্রবার",
            BengaliWeekDays::Shonibar => "শনিবার",
        };
        write!(f, "{}", day_str)
    }
}
