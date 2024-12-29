use ponjika::*;
fn main() {
    let date = EnglishDate::create_date(10, EnglishMonths::October, 2010);

    match date {
        Some(english_date) => {
            let bengali_date = get_bengali_date_from_gregorian(english_date);
            match bengali_date {
                Date::Invalid => {
                    eprintln!("Failed to convert to Bengali date");
                    return;
                }
                _ => {
                    println!("{}", format_date(bengali_date));
                }
            }
        }
        None => {
            eprintln!("The date is not a valid greogrian date");
            return;
        }
    }
}
