pub enum BengaliMonths {
    Baishakh,
    Jestha,
    Ashad,
    Shrawan,
    Bhadra,
    Ashwin,
    Kartik,
    Mangsir,
    Poush,
    Magh,
    Falgun,
    Chaitra,
}

impl BengaliMonths {
    pub fn as_str(&self) -> &str {
        match self {
            BengaliMonths::Baishakh => "বৈশাখ",
            BengaliMonths::Jestha => "জ্যেষ্ঠ",
            BengaliMonths::Ashad => "আষাঢ়",
            BengaliMonths::Shrawan => "শ্রাবণ",
            BengaliMonths::Bhadra => "ভাদ্র",
            BengaliMonths::Ashwin => "আশ্বিন",
            BengaliMonths::Kartik => "কার্তিক",
            BengaliMonths::Mangsir => "মার্গশির",
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

pub enum EnglishDays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
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