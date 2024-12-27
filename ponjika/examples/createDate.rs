use ponjika::*;
fn main() {
    let today = get_today_bengali_calendar();
    println!(
        "Today is: {} {} {} {}",
        today.get_day(),
        today.get_week_day(),
        today.get_month(),
        today.get_year()
    );
}
