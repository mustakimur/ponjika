use ponjika::*;
fn main() {
    let date = get_bengali_date_from_gregorian(30, 3, 2012);

    match date {
        Date::Invalid => {
            eprintln!("Invalid date");
            return;
        }
        _ => {
            println!(
                "Date is: {1}, {0} {2} {3}",
                date.get_day(),
                date.get_week_day(),
                date.get_month(),
                date.get_year()
            );
        }
    }
}
