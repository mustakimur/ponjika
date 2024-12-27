pub enum Month {
    English(EnglishMonths),
    Bengali(BengaliMonths),
    Invalid,
}

impl Month {
    pub fn get_name(&self) -> &str {
        match self {
            Month::English(month) => month.get_name(),
            Month::Bengali(month) => month.get_name(),
            Month::Invalid => "MonthError: The month in the date was wrong",
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
    pub fn get_month(month: u8) -> Month {
        match month {
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
            _ => Month::Invalid,
        }
    }

    pub fn get_name(&self) -> &str {
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
    pub fn get_month(month: u8) -> Month {
        match month {
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
            _ => Month::Invalid,
        }
    }

    pub fn get_name(&self) -> &str {
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
