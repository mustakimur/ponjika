use ponjika::*;
fn main() {
    let today = get_today_bengali_calendar();
    match today {
        Date::Invalid => {
            eprintln!("Invalid date");
            return;
        }
        _ => {
            println!(
                "Today is: {1}, {0} {2} {3}",
                today.get_day(),
                today.get_week_day(),
                today.get_month(),
                today.get_year()
            );
        }
    }
}