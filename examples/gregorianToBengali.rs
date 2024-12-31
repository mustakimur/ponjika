use ponjika::*;
fn main() {
    let date = EnglishDate::create_date(30, EnglishMonths::November, 2010);

    match date {
        Ok(english_date) => {
            let bengali_date = get_bengali_date_from_gregorian(english_date);
            match bengali_date {
                Ok(date) => {
                    println!("{}", date.to_string());
                }
                Err(convert_err) => {
                    eprintln!("Failed to convert to Bengali date: {:?}", convert_err);
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
