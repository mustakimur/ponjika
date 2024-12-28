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
    /// assert_eq!(month.get_name(), "January");
    /// let month = Month::Bengali(BengaliMonths::Baishakh);
    /// assert_eq!(month.get_name(), "বৈশাখ");
    /// ```
    /// # Note
    /// * The function will return the name of the month
    /// * The function will return "MonthError: The month in the date was wrong" if the month is invalid
    ///
    pub fn get_name(&self) -> &str {
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
}
