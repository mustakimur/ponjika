use ponjika::*;
fn main() {
    let date = EnglishDate::create_date(10, EnglishMonths::October, 2010);

    match date {
        Ok(english_date) => {
            let bengali_date = get_bengali_date_from_gregorian(english_date);
            match bengali_date {
                Ok(date) => {
                    println!("{}", date.to_string());
                }
                Err(_) => {
                    eprintln!("Failed to convert to Bengali date");
                    return;
                }
            }
        }
        Err(_) => {
            eprintln!("The date is not a valid greogrian date");
            return;
        }
    }
}
