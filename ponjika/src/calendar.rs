pub enum BengaliMonths {
    Baishakh,
    Jestha,
    Ashad,
    Shrawan,
    Bhadra,
    Ashwin,
    Kartik,
    Ogrohaeon,
    Poush,
    Magh,
    Falgun,
    Chaitra,
}

impl BengaliMonths {
    pub fn from_index(month: u8) -> BengaliMonths {
        match month {
            1 => BengaliMonths::Baishakh,
            2 => BengaliMonths::Jestha,
            3 => BengaliMonths::Ashad,
            4 => BengaliMonths::Shrawan,
            5 => BengaliMonths::Bhadra,
            6 => BengaliMonths::Ashwin,
            7 => BengaliMonths::Kartik,
            8 => BengaliMonths::Ogrohaeon,
            9 => BengaliMonths::Poush,
            10 => BengaliMonths::Magh,
            11 => BengaliMonths::Falgun,
            12 => BengaliMonths::Chaitra,
            _ => BengaliMonths::Baishakh,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BengaliMonths::Baishakh => "বৈশাখ",
            BengaliMonths::Jestha => "জ্যেষ্ঠ",
            BengaliMonths::Ashad => "আষাঢ়",
            BengaliMonths::Shrawan => "শ্রাবণ",
            BengaliMonths::Bhadra => "ভাদ্র",
            BengaliMonths::Ashwin => "আশ্বিন",
            BengaliMonths::Kartik => "কার্তিক",
            BengaliMonths::Ogrohaeon => "অগ্রহায়ণ",
            BengaliMonths::Poush => "পৌষ",
            BengaliMonths::Magh => "মাঘ",
            BengaliMonths::Falgun => "ফাল্গুন",
            BengaliMonths::Chaitra => "চৈত্র",
        }
    }
}

pub enum EnglishMonths {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl EnglishMonths {
    pub fn from_index(month: u8) -> EnglishMonths {
        match month {
            1 => EnglishMonths::January,
            2 => EnglishMonths::February,
            3 => EnglishMonths::March,
            4 => EnglishMonths::April,
            5 => EnglishMonths::May,
            6 => EnglishMonths::June,
            7 => EnglishMonths::July,
            8 => EnglishMonths::August,
            9 => EnglishMonths::September,
            10 => EnglishMonths::October,
            11 => EnglishMonths::November,
            12 => EnglishMonths::December,
            _ => EnglishMonths::January,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            EnglishMonths::January => "January",
            EnglishMonths::February => "February",
            EnglishMonths::March => "March",
            EnglishMonths::April => "April",
            EnglishMonths::May => "May",
            EnglishMonths::June => "June",
            EnglishMonths::July => "July",
            EnglishMonths::August => "August",
            EnglishMonths::September => "September",
            EnglishMonths::October => "October",
            EnglishMonths::November => "November",
            EnglishMonths::December => "December",
        }
    }
}

pub enum EnglishDays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl EnglishDays {
    pub fn as_str(&self) -> &str {
        match self {
            EnglishDays::Sunday => "Sunday",
            EnglishDays::Monday => "Monday",
            EnglishDays::Tuesday => "Tuesday",
            EnglishDays::Wednesday => "Wednesday",
            EnglishDays::Thursday => "Thursday",
            EnglishDays::Friday => "Friday",
            EnglishDays::Saturday => "Saturday",
        }
    }
    
}

pub enum BengaliDays {
    Robibar,
    Sombar,
    Mongolbar,
    Budhbar,
    Brihoshpotibar,
    Shukrobar,
    Shonibar,
}

impl BengaliDays {
    pub fn map_english(week_day: EnglishDays) -> BengaliDays {
        match week_day {
            EnglishDays::Sunday => BengaliDays::Robibar,
            EnglishDays::Monday => BengaliDays::Sombar,
            EnglishDays::Tuesday => BengaliDays::Mongolbar,
            EnglishDays::Wednesday => BengaliDays::Budhbar,
            EnglishDays::Thursday => BengaliDays::Brihoshpotibar,
            EnglishDays::Friday => BengaliDays::Shukrobar,
            EnglishDays::Saturday => BengaliDays::Shonibar,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            BengaliDays::Robibar => "রবিবার",
            BengaliDays::Sombar => "সোমবার",
            BengaliDays::Mongolbar => "মঙ্গলবার",
            BengaliDays::Budhbar => "বুধবার",
            BengaliDays::Brihoshpotibar => "বৃহস্পতিবার",
            BengaliDays::Shukrobar => "শুক্রবার",
            BengaliDays::Shonibar => "শনিবার",
        }
    }
}

pub struct EnglishDate {
    pub day: u8,
    pub week_day: EnglishDays,
    pub month: u8,
    pub month_name: EnglishMonths,
    pub year: u16,
}

pub struct BengaliDate {
    pub day: u8,
    pub week_day: BengaliDays,
    pub month: u8,
    pub month_name: BengaliMonths,
    pub year: u16,
}

impl EnglishDate {
    pub fn new(day: u8, week_day: EnglishDays, month: u8, year: u16) -> EnglishDate {
        EnglishDate {
            day,
            week_day,
            month,
            month_name: EnglishMonths::January,
            year,
        }
    }
}

impl BengaliDate {
    pub fn new(day: u8, week_day: BengaliDays, month: u8, year: u16) -> BengaliDate {
        BengaliDate {
            day,
            week_day,
            month,
            month_name: BengaliMonths::Baishakh,
            year,
        }
    }
}