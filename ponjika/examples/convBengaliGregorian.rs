use ponjika::*;
fn main() {
    let date = get_bengali_date_from_gregorian(30, 3, 2012);

    match date {
        Date::Invalid => {
            eprintln!("Invalid date");
            return;
        }
        _ => {
            println!("{}", format_date(date));
        }
    }
}
