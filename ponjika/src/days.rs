pub enum WeekDays {
    English(EnglishWeekDays),
    Bengali(BengaliWeekDays),
    Invalid,
}

impl WeekDays {
    pub fn get_name(&self) -> &str {
        match self {
            WeekDays::English(day) => day.get_name(),
            WeekDays::Bengali(day) => day.get_name(),
            WeekDays::Invalid => "WeekDayError: The week day in the date was wrong",
        }
    }
}

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

    pub fn map_bengali(week_day: BengaliWeekDays) -> EnglishWeekDays {
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

    pub fn map_english(week_day: EnglishWeekDays) -> BengaliWeekDays {
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
