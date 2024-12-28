//! # Days: The week day names in English and Bengali
//! The `days` module provides the week day names in English and Bengali.
//! The `WeekDays` enum is used to represent both English and Bengali week days.
//! The `EnglishWeekDays` and `BengaliWeekDays` enum variants are the English and Bengali week days respectively.

/// The enum `WeekDays` is used to represent both English and Bengali week days.
/// The invalid variant is used when the week day is invalid.
pub enum WeekDays {
    English(EnglishWeekDays),
    Bengali(BengaliWeekDays),
    Invalid,
}

impl WeekDays {
    /// Get the week day name of the selected week day
    /// # Returns
    /// * `&str` - The name of the week day
    /// # Example
    /// ```
    /// use ponjika::days::{WeekDays, EnglishWeekDays, BengaliWeekDays};
    /// let week_day = WeekDays::English(EnglishWeekDays::Sunday);
    /// assert_eq!(week_day.get_name(), "Sunday");
    /// let week_day = WeekDays::Bengali(BengaliWeekDays::Robibar);
    /// assert_eq!(week_day.get_name(), "রবিবার");
    /// ```
    /// # Note
    /// * The function will return the name of the week day
    /// * The function will return "WeekDayError: The week day in the date was wrong" if the week day is invalid
    pub fn get_name(&self) -> &str {
        match self {
            WeekDays::English(day) => day.get_name(),
            WeekDays::Bengali(day) => day.get_name(),
            WeekDays::Invalid => "WeekDayError: The week day in the date was wrong",
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
    /// Get the week day name of the selected week day
    /// # Returns
    /// * `&str` - The name of the week day
    /// # Example
    /// ```
    /// use ponjika::days::EnglishWeekDays;
    /// let week_day = EnglishWeekDays::Sunday;
    /// assert_eq!(week_day.get_name(), "Sunday");
    /// ```
    /// # Note
    /// * The function will return the name of the week day
    pub fn get_name(&self) -> &str {
        match self {
            EnglishWeekDays::Sunday => "Sunday",
            EnglishWeekDays::Monday => "Monday",
            EnglishWeekDays::Tuesday => "Tuesday",
            EnglishWeekDays::Wednesday => "Wednesday",
            EnglishWeekDays::Thursday => "Thursday",
            EnglishWeekDays::Friday => "Friday",
            EnglishWeekDays::Saturday => "Saturday",
        }
    }

    fn map_bengali(week_day: BengaliWeekDays) -> EnglishWeekDays {
        match week_day {
            BengaliWeekDays::Robibar => EnglishWeekDays::Sunday,
            BengaliWeekDays::Sombar => EnglishWeekDays::Monday,
            BengaliWeekDays::Mongolbar => EnglishWeekDays::Tuesday,
            BengaliWeekDays::Budhbar => EnglishWeekDays::Wednesday,
            BengaliWeekDays::Brihoshpotibar => EnglishWeekDays::Thursday,
            BengaliWeekDays::Shukrobar => EnglishWeekDays::Friday,
            BengaliWeekDays::Shonibar => EnglishWeekDays::Saturday,
        }
    }

    /// Map the Bengali week day name to the English week day name
    /// # Arguments
    /// * `week_day` - &str
    /// # Returns
    /// * `EnglishWeekDays` - The English week day
    /// # Example
    /// ```
    /// use ponjika::days::EnglishWeekDays;
    /// let week_day = EnglishWeekDays::map_bengali_name("রবিবার");
    /// match week_day {
    ///   EnglishWeekDays::Sunday => println!("Sunday"),
    ///  _ => println!("Other days"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the English week day
    pub fn map_bengali_name(week_day: &str) -> EnglishWeekDays {
        match week_day {
            "রবিবার" => EnglishWeekDays::Sunday,
            "সোমবার" => EnglishWeekDays::Monday,
            "মঙ্গলবার" => EnglishWeekDays::Tuesday,
            "বুধবার" => EnglishWeekDays::Wednesday,
            "বৃহস্পতিবার" => EnglishWeekDays::Thursday,
            "শুক্রবার" => EnglishWeekDays::Friday,
            "শনিবার" => EnglishWeekDays::Saturday,
            _ => EnglishWeekDays::Sunday,
        }
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
    /// Get the week day name of the selected week day
    /// # Returns
    /// * `&str` - The name of the week day
    /// # Example
    /// ```
    /// use ponjika::days::BengaliWeekDays;
    /// let week_day = BengaliWeekDays::Robibar;
    /// assert_eq!(week_day.get_name(), "রবিবার");
    /// ```
    /// # Note
    /// * The function will return the name of the week day
    pub fn get_name(&self) -> &str {
        match self {
            BengaliWeekDays::Robibar => "রবিবার",
            BengaliWeekDays::Sombar => "সোমবার",
            BengaliWeekDays::Mongolbar => "মঙ্গলবার",
            BengaliWeekDays::Budhbar => "বুধবার",
            BengaliWeekDays::Brihoshpotibar => "বৃহস্পতিবার",
            BengaliWeekDays::Shukrobar => "শুক্রবার",
            BengaliWeekDays::Shonibar => "শনিবার",
        }
    }

    fn map_english(week_day: EnglishWeekDays) -> BengaliWeekDays {
        match week_day {
            EnglishWeekDays::Sunday => BengaliWeekDays::Robibar,
            EnglishWeekDays::Monday => BengaliWeekDays::Sombar,
            EnglishWeekDays::Tuesday => BengaliWeekDays::Mongolbar,
            EnglishWeekDays::Wednesday => BengaliWeekDays::Budhbar,
            EnglishWeekDays::Thursday => BengaliWeekDays::Brihoshpotibar,
            EnglishWeekDays::Friday => BengaliWeekDays::Shukrobar,
            EnglishWeekDays::Saturday => BengaliWeekDays::Shonibar,
        }
    }

    /// Map the English week day name to the Bengali week day name
    /// # Arguments
    /// * `week_day` - &str
    /// # Returns
    /// * `BengaliWeekDays` - The Bengali week day
    /// # Example
    /// ```
    /// use ponjika::days::BengaliWeekDays;
    /// let week_day = BengaliWeekDays::map_english_name("Sunday");
    /// match week_day {
    ///    BengaliWeekDays::Robibar => println!("রবিবার"),
    ///   _ => println!("Other days"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the Bengali week day
    pub fn map_english_name(week_day: &str) -> BengaliWeekDays {
        match week_day {
            "Sunday" => BengaliWeekDays::Robibar,
            "Monday" => BengaliWeekDays::Sombar,
            "Tuesday" => BengaliWeekDays::Mongolbar,
            "Wednesday" => BengaliWeekDays::Budhbar,
            "Thursday" => BengaliWeekDays::Brihoshpotibar,
            "Friday" => BengaliWeekDays::Shukrobar,
            "Saturday" => BengaliWeekDays::Shonibar,
            _ => BengaliWeekDays::Robibar,
        }
    }
}
