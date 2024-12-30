use ponjika::*;
fn main() {
    let today = get_today_bengali_calendar();
    match today {
        Date::Unknown => {
            eprintln!("The system return an invalid date");
            return;
        }
        _ => {
            //println!("{}", format_date_with_weekday(today));
        }
    }
}
