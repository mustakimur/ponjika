use ponjika::{get_today_bengali_calendar, BengaliDate, BengaliDays, BengaliMonths};
fn main(){
    let today = get_today_bengali_calendar();
    println!("Today is: {} {} {} {}", today.day, today.week_day.as_str(), today.month.as_str(), today.year);
}