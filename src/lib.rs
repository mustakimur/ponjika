pub mod calendar;
pub mod date;
pub mod days;
pub mod months;
mod season;

pub use calendar::*;
pub use date::*;
pub use days::*;
pub use months::*;

mod tests {
    use super::*;

    #[test]
    fn test_conv_bengali_01() {
        let english_date = EnglishDate::create_date(10, EnglishMonths::October, 2010);
        match english_date {
            Ok(e_date) => {
                let bengali_date = calendar::get_bengali_date_from_gregorian(e_date);
                match bengali_date {
                    Ok(b_date) => {
                        assert_eq!(
                            (
                                "২৫".to_string(),
                                "রবিবার".to_string(),
                                "আশ্বিন".to_string(),
                                "১৪১৭".to_string()
                            ),
                            b_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_bengali_02() {
        let english_date = EnglishDate::create_date(3, EnglishMonths::April, 2012);
        match english_date {
            Ok(e_date) => {
                let bengali_date = calendar::get_bengali_date_from_gregorian(e_date);
                match bengali_date {
                    Ok(b_date) => {
                        assert_eq!(
                            (
                                "২০".to_string(),
                                "মঙ্গলবার".to_string(),
                                "চৈত্র".to_string(),
                                "১৪১৮".to_string()
                            ),
                            b_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_bengali_03() {
        let english_date = EnglishDate::create_date(31, EnglishMonths::July, 2000);
        match english_date {
            Ok(e_date) => {
                let bengali_date = calendar::get_bengali_date_from_gregorian(e_date);
                match bengali_date {
                    Ok(b_date) => {
                        assert_eq!(
                            (
                                "১৬".to_string(),
                                "সোমবার".to_string(),
                                "শ্রাবণ".to_string(),
                                "১৪০৭".to_string()
                            ),
                            b_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_bengali_04() {
        let english_date = EnglishDate::create_date(15, EnglishMonths::January, 1998);
        match english_date {
            Ok(e_date) => {
                let bengali_date = calendar::get_bengali_date_from_gregorian(e_date);
                match bengali_date {
                    Ok(b_date) => {
                        assert_eq!(
                            (
                                "১".to_string(),
                                "বৃহস্পতিবার".to_string(),
                                "মাঘ".to_string(),
                                "১৪০৪".to_string()
                            ),
                            b_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_bengali_05() {
        let english_date = EnglishDate::create_date(4, EnglishMonths::July, 1988);
        match english_date {
            Ok(e_date) => {
                let bengali_date = calendar::get_bengali_date_from_gregorian(e_date);
                match bengali_date {
                    Ok(b_date) => {
                        assert_eq!(
                            (
                                "২০".to_string(),
                                "সোমবার".to_string(),
                                "আষাঢ়".to_string(),
                                "১৩৯৫".to_string()
                            ),
                            b_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_english_01() {
        let bengali_date = BengaliDate::create_date(4, BengaliMonths::Ashwin, 1409);
        match bengali_date {
            Ok(b_date) => {
                let english_date = calendar::get_gregorian_date_from_bengali(b_date);
                match english_date {
                    Ok(e_date) => {
                        assert_eq!(
                            (
                                "19".to_string(),
                                "Thursday".to_string(),
                                "September".to_string(),
                                "2002".to_string()
                            ),
                            e_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_english_02() {
        let bengali_date = BengaliDate::create_date(31, BengaliMonths::Baishakh, 1419);
        match bengali_date {
            Ok(b_date) => {
                let english_date = calendar::get_gregorian_date_from_bengali(b_date);
                match english_date {
                    Ok(e_date) => {
                        assert_eq!(
                            (
                                "14".to_string(),
                                "Monday".to_string(),
                                "May".to_string(),
                                "2012".to_string()
                            ),
                            e_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_conv_english_03() {
        let bengali_date = BengaliDate::create_date(25, BengaliMonths::Shrawan, 1435);
        match bengali_date {
            Ok(b_date) => {
                let english_date = calendar::get_gregorian_date_from_bengali(b_date);
                match english_date {
                    Ok(e_date) => {
                        assert_eq!(
                            (
                                "9".to_string(),
                                "Wednesday".to_string(),
                                "August".to_string(),
                                "2028".to_string()
                            ),
                            e_date.get_date().unwrap()
                        );
                    }
                    Err(_) => {
                        assert!(false);
                    }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
