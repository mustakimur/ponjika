//! # Months: The module to represent the months
//! This module contains the enum `Month` and the enum `EnglishMonths` and `BengaliMonths`.
//! The `Month` enum is used to represent both English and Bengali months.
//! The `EnglishMonths` and `BengaliMonths` enum variants are the English and Bengali months respectively.

/// The enum `Month` is used to represent both English and Bengali months.
/// The invalid variant is used when the month is invalid.
#[derive(Debug)]
pub enum Month {
    English(EnglishMonths),
    Bengali(BengaliMonths),
    Invalid,
}

impl Month {
    /// Get the month name of the selected month
    /// # Returns
    /// * `&str` - The name of the month
    /// # Example
    /// ```
    /// use ponjika::months::{Month, EnglishMonths, BengaliMonths};
    /// let month = Month::English(EnglishMonths::January);
    /// assert_eq!(month.get_month_name(), "January");
    /// let month = Month::Bengali(BengaliMonths::Baishakh);
    /// assert_eq!(month.get_month_name(), "বৈশাখ");
    /// ```
    /// # Note
    /// * The function will return the name of the month
    /// * The function will return "MonthError: The month in the date was wrong" if the month is invalid
    ///
    pub fn get_month_name(&self) -> &str {
        match self {
            Month::English(month) => month.get_name(),
            Month::Bengali(month) => month.get_name(),
            Month::Invalid => "MonthError: The month in the date was wrong",
        }
    }
}

/// The enum `EnglishMonths` is used to represent the English months.
/// The enum variants are the English months.
#[derive(Debug)]
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
    fn get_name(&self) -> &str {
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

    /// Get the enum `EnglishMonths` variant of the selected month
    /// # Arguments
    /// * `month` - u8
    /// # Returns
    /// * `Month` - The EnglishMonth enum variant of the selected month
    /// # Example
    /// ```
    /// use ponjika::months::{Month, EnglishMonths};
    /// let month = EnglishMonths::get_month(1);
    /// match month {
    ///     Month::English(month) => println!("The month is {:?}", month),
    ///     Month::Bengali(_) => println!("The month is Bengali"),
    ///     _ => println!("The month is invalid"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the `EnglishMonth` enum variant of the selected month
    /// * The function will return `Month::Invalid` if the month is invalid
    pub fn get_month(month: u8) -> Option<EnglishMonths> {
        match month {
            1 => Some(EnglishMonths::January),
            2 => Some(EnglishMonths::February),
            3 => Some(EnglishMonths::March),
            4 => Some(EnglishMonths::April),
            5 => Some(EnglishMonths::May),
            6 => Some(EnglishMonths::June),
            7 => Some(EnglishMonths::July),
            8 => Some(EnglishMonths::August),
            9 => Some(EnglishMonths::September),
            10 => Some(EnglishMonths::October),
            11 => Some(EnglishMonths::November),
            12 => Some(EnglishMonths::December),
            _ => None,
        }
    }

    pub fn map_month_to_index(&self) -> u8 {
        match self {
            EnglishMonths::January => 1,
            EnglishMonths::February => 2,
            EnglishMonths::March => 3,
            EnglishMonths::April => 4,
            EnglishMonths::May => 5,
            EnglishMonths::June => 6,
            EnglishMonths::July => 7,
            EnglishMonths::August => 8,
            EnglishMonths::September => 9,
            EnglishMonths::October => 10,
            EnglishMonths::November => 11,
            EnglishMonths::December => 12,
        }
    }
}

/// The enum `BengaliMonths` is used to represent the Bengali months.
/// The enum variants are the Bengali months.
#[derive(Debug)]
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
    fn get_name(&self) -> &str {
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

    /// Get the enum `BengaliMonths` variant of the selected month
    /// # Arguments
    /// * `month` - u8
    /// # Returns
    /// * `Month` - The BengaliMonth enum variant of the selected month
    /// # Example
    /// ```
    /// use ponjika::months::{Month, BengaliMonths};
    /// let month = BengaliMonths::get_month(1);
    /// match month {
    ///    Month::English(_) => println!("The month is English"),
    ///   Month::Bengali(month) => println!("The month is {:?}", month),
    ///    _ => println!("The month is invalid"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the `BengaliMonth` enum variant of the selected month
    /// * The function will return `Month::Invalid` if the month is invalid
    pub fn get_month(month: u8) -> Option<BengaliMonths> {
        match month {
            1 => Some(BengaliMonths::Baishakh),
            2 => Some(BengaliMonths::Jestha),
            3 => Some(BengaliMonths::Ashad),
            4 => Some(BengaliMonths::Shrawan),
            5 => Some(BengaliMonths::Bhadra),
            6 => Some(BengaliMonths::Ashwin),
            7 => Some(BengaliMonths::Kartik),
            8 => Some(BengaliMonths::Ogrohaeon),
            9 => Some(BengaliMonths::Poush),
            10 => Some(BengaliMonths::Magh),
            11 => Some(BengaliMonths::Falgun),
            12 => Some(BengaliMonths::Chaitra),
            _ => None,
        }
    }

    pub fn map_month_to_index(&self) -> u8 {
        match self {
            BengaliMonths::Baishakh => 1,
            BengaliMonths::Jestha => 2,
            BengaliMonths::Ashad => 3,
            BengaliMonths::Shrawan => 4,
            BengaliMonths::Bhadra => 5,
            BengaliMonths::Ashwin => 6,
            BengaliMonths::Kartik => 7,
            BengaliMonths::Ogrohaeon => 8,
            BengaliMonths::Poush => 9,
            BengaliMonths::Magh => 10,
            BengaliMonths::Falgun => 11,
            BengaliMonths::Chaitra => 12,
        }
    }
}
