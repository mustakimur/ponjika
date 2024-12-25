mod calendar;

pub use calendar::EnglishMonths;

pub struct Date {
    day: u8,
    month: EnglishMonths,
    year: u16,
}

impl Date {
    pub fn new(day: u8, month: EnglishMonths, year: u16) -> Date {
        Date { day, month, year }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {}
}
