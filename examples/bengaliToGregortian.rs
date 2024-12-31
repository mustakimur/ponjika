use ponjika::*;

fn main() {
    let bengali_date = BengaliDate::create_date(3, BengaliMonths::Falgun, 1430);

    match bengali_date {
        Ok(bengali_date) => {
            let gregorian_date = get_gregorian_date_from_bengali(bengali_date);

            match gregorian_date {
                Ok(date) => {
                    println!("Gregorian Date: {}", date.to_string());
                }
                Err(convert_err) => {
                    eprintln!("Failed to convert to Gregorian date: {:?}", convert_err);
                    return;
                }
            }
        }
        Err(err) => {
            eprintln!("DateError: {:?}", err);
            return;
        }
    }

    let bengali_date = BengaliDate::create_date(12, BengaliMonths::Shrawan, 1431);

    match bengali_date {
        Ok(bengali_date) => {
            let gregorian_date = get_gregorian_date_from_bengali(bengali_date);

            match gregorian_date {
                Ok(date) => {
                    println!("Gregorian Date: {}", date.to_string());
                }
                Err(convert_err) => {
                    eprintln!("Failed to convert to Gregorian date: {:?}", convert_err);
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
