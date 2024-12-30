use ponjika::*;
fn main() {
    let today = get_today_bengali_date();
    match today {
        Ok(bengali_date) => {
            println!("{}", bengali_date.to_string());
        }
        Err(_) => {
            eprintln!("The date is not a valid greogrian date");
            return;
        }
    }
}
