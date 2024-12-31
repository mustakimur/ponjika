#[macro_use]
extern crate afl;

use ponjika::*;

fn convert_to_u16(s: Option<&&str>) -> u16 {
    match s {
        Some(s) => {
            let s = match s.parse::<u16>() {
                Ok(s) => s,
                Err(err) => 0,
            };
            s
        }
        None => 0,
    }
}

fn convert_to_u8(s: Option<&&str>) -> u8 {
    match s {
        Some(s) => {
            let s = match s.parse::<u8>() {
                Ok(s) => s,
                Err(err) => 0,
            };
            s
        }
        None => 0,
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let data: Vec<&str> = s.split_whitespace().collect();
            let day = convert_to_u8(data.get(0));
            let month = convert_to_u8(data.get(1));
            let year = convert_to_u16(data.get(2));

            let month = BengaliMonths::get_month(month);

            match month {
                Ok(month) => {
                    let date = BengaliDate::create_date(day, month, year);

                    match date {
                        Ok(bengali_date) => {
                            println!("{:?}", bengali_date.to_string());
                            let english_date = get_gregorian_date_from_bengali(bengali_date);
                            match english_date {
                                Ok(date) => {
                                    println!("{}", date.to_string());
                                }
                                Err(convert_err) => {
                                    eprintln!(
                                        "Failed to convert to English date: {:?}",
                                        convert_err
                                    );
                                    return;
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("DateError: {:?}", err);
                            return;
                        }
                    }
                }
                _ => {
                    eprintln!("InvalidMonthError: {:?}", month);
                }
            }
        }
    });
}
