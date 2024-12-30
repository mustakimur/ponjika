use ponjika::*;
fn main() {
    let today = get_today_bengali_date();
    match today {
        Ok(bengali_date) => {
            println!("{}", bengali_date.to_string());
        }
        Err(err) => {
            eprintln!("Date error: {:?}", err);
        }
    }
}
