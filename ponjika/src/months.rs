//! # Months: The module to represent the months
//! The `months` module provides the English and Bengali months.
//! The `Month` enum is used to represent both English and Bengali months.
//! The `EnglishMonths` and `BengaliMonths` enum variants are the English and Bengali months respectively.
//! The `MonthError` enum is used to represent the error when the month is invalid.

use std::fmt;

#[derive(Debug)]
pub enum MonthError {
    UnknownMonth,
    WrongRange,
}

impl std::fmt::Display for MonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MonthError::UnknownMonth => write!(f, "MonthError: Unknown month"),
            MonthError::WrongRange => write!(f, "MonthError: Month should be between 1 and 12"),
        }
    }
}

type Result<T> = std::result::Result<T, MonthError>;

/// The enum `Month` is used to represent both English and Bengali months.
/// The invalid variant is used when the month is invalid.
#[derive(Debug)]
pub enum Month {
    English(EnglishMonths),
    Bengali(BengaliMonths),
    Unknown,
}

impl Month {
    /// Get the month name of the selected month
    /// # Returns
    /// * `Result<String>` - The name of the month
    /// # Example
    /// ```
    /// use ponjika::months::{Month, EnglishMonths, BengaliMonths};
    /// let month = Month::English(EnglishMonths::January);
    /// assert_eq!(month.get_month_name().unwrap(), "January");
    /// let month = Month::Bengali(BengaliMonths::Baishakh);
    /// assert_eq!(month.get_month_name().unwrap(), "বৈশাখ");
    /// ```
    /// # Note
    /// * The function will return the name of the month
    /// * The function will return "MonthError: Unknown month" if the month is invalid
    pub fn get_month_name(&self) -> Result<String> {
        match self {
            Month::English(month) => Ok(month.to_string()),
            Month::Bengali(month) => Ok(month.to_string()),
            Month::Unknown => Err(MonthError::UnknownMonth),
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
    ///  Ok(month) => println!("The month is {:?}", month),
    ///  Err(_) => println!("The month is invalid"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the `EnglishMonth` enum variant of the selected month
    /// * The function will return None if the month is invalid
    pub fn get_month(month: u8) -> Result<Self> {
        match month {
            1 => Ok(EnglishMonths::January),
            2 => Ok(EnglishMonths::February),
            3 => Ok(EnglishMonths::March),
            4 => Ok(EnglishMonths::April),
            5 => Ok(EnglishMonths::May),
            6 => Ok(EnglishMonths::June),
            7 => Ok(EnglishMonths::July),
            8 => Ok(EnglishMonths::August),
            9 => Ok(EnglishMonths::September),
            10 => Ok(EnglishMonths::October),
            11 => Ok(EnglishMonths::November),
            12 => Ok(EnglishMonths::December),
            _ => Err(MonthError::WrongRange),
        }
    }

    /// Map the English month to the index
    /// # Returns
    /// * `u8` - The index of the month
    /// # Example
    /// ```
    /// use ponjika::months::EnglishMonths;
    /// let month = EnglishMonths::January;
    /// println!("{}", month.map_to_index());
    /// ```
    /// # Note
    /// * The function will return the index of the month
    /// * The index of the month will be between 1 and 12
    pub fn map_to_index(&self) -> u8 {
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

impl fmt::Display for EnglishMonths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnglishMonths::January => write!(f, "January"),
            EnglishMonths::February => write!(f, "February"),
            EnglishMonths::March => write!(f, "March"),
            EnglishMonths::April => write!(f, "April"),
            EnglishMonths::May => write!(f, "May"),
            EnglishMonths::June => write!(f, "June"),
            EnglishMonths::July => write!(f, "July"),
            EnglishMonths::August => write!(f, "August"),
            EnglishMonths::September => write!(f, "September"),
            EnglishMonths::October => write!(f, "October"),
            EnglishMonths::November => write!(f, "November"),
            EnglishMonths::December => write!(f, "December"),
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
    ///  Ok(month) => println!("The month is {:?}", month),
    ///  Err(_) => println!("The month is invalid"),
    /// }
    /// ```
    /// # Note
    /// * The function will return the `BengaliMonth` enum variant of the selected month
    /// * The function will return None if the month is invalid
    pub fn get_month(month: u8) -> Result<Self> {
        match month {
            1 => Ok(BengaliMonths::Baishakh),
            2 => Ok(BengaliMonths::Jestha),
            3 => Ok(BengaliMonths::Ashad),
            4 => Ok(BengaliMonths::Shrawan),
            5 => Ok(BengaliMonths::Bhadra),
            6 => Ok(BengaliMonths::Ashwin),
            7 => Ok(BengaliMonths::Kartik),
            8 => Ok(BengaliMonths::Ogrohaeon),
            9 => Ok(BengaliMonths::Poush),
            10 => Ok(BengaliMonths::Magh),
            11 => Ok(BengaliMonths::Falgun),
            12 => Ok(BengaliMonths::Chaitra),
            _ => Err(MonthError::WrongRange),
        }
    }

    /// Map the Bengali month to the index
    /// # Returns
    /// * `u8` - The index of the month
    /// # Example
    /// ```
    /// use ponjika::months::BengaliMonths;
    /// let month = BengaliMonths::Baishakh;
    /// println!("{}", month.map_to_index());
    /// ```
    /// # Note
    /// * The function will return the index of the month
    /// * The index of the month will be between 1 and 12
    pub fn map_to_index(&self) -> u8 {
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

impl fmt::Display for BengaliMonths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BengaliMonths::Baishakh => write!(f, "বৈশাখ"),
            BengaliMonths::Jestha => write!(f, "জ্যৈষ্ঠ"),
            BengaliMonths::Ashad => write!(f, "আষাঢ়"),
            BengaliMonths::Shrawan => write!(f, "শ্রাবণ"),
            BengaliMonths::Bhadra => write!(f, "ভাদ্র"),
            BengaliMonths::Ashwin => write!(f, "আশ্বিন"),
            BengaliMonths::Kartik => write!(f, "কার্তিক"),
            BengaliMonths::Ogrohaeon => write!(f, "অগ্রহায়ণ"),
            BengaliMonths::Poush => write!(f, "পৌষ"),
            BengaliMonths::Magh => write!(f, "মাঘ"),
            BengaliMonths::Falgun => write!(f, "ফাল্গুন"),
            BengaliMonths::Chaitra => write!(f, "চৈত্র"),
        }
    }
}
