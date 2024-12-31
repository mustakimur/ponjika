# Bengali Calendar (Ponjika)

[![Crates.io](https://img.shields.io/crates/v/ponjika.svg)](https://crates.io/crates/ponjika)
[![Rust](https://github.com/mustakimur/ponjika/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/mustakimur/ponjika/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/ponjika/badge.svg)](https://docs.rs/ponjika)
[![License](https://img.shields.io/crates/l/ponjika.svg)](https://crates.io/crates/ponjikae)

Welcome to `Ponjika` and the year 2025. This crate will support many features relevant to the Bengali (Bangladeshi) calendar. Our goal is to minimize external resource dependency and safely compute Bengali dates, seasons, holidays, etc. If you have a feature idea that aligns with the crate's purpose, please create an issue with the tag `[Feature]` in the issue title. If you have found a bug, please create an issue with the tag `[Bug]` in the issue title.

## FAQ
**Q1: Can I create a Gregorian date and convert it to a Bengali date?**
- First, you have to create an `EnglishDate`. For example: `let date = EnglishDate::create_date(30, EnglishMonths::November, 2010);`
- Finally, if your date is valid, you can get the respective Bengali date using: `let bengali_date = get_bengali_date_from_gregorian(english_date);`
- In both steps, the crate may return `DateError` for many different reasons including invalid date, arithmetic operation, casting, etc. So, make sure you do handle all the possible error cases.
- Example: `examples/gregorianToBengali.rs`.

**Q2: Can I create a Bengali date and convert it to a Gregorian date?**
- First, you have to create an `BengaliDate`. For example: `let bengali_date = BengaliDate::create_date(3, BengaliMonths::Falgun, 1430);`
- Finally, if your date is valid, you can get the respective gregorian date using: `let english_date = get_gregorian_date_from_bengali(bengali_date);`
- In both steps, the crate may return `DateError` for many different reasons including invalid date, arithmetic operation, casting, etc. So, make sure you do handle all the possible error cases.
- Example: `examples/bengaliToGregortian.rs`.

## Fuzzing
The project currently support afl.rs fuzzing. Take a look into `fuzz` directory. To run the fuzzer:
```
cd fuzz
cargo afl build
cargo afl fuzz -i in/english_create_date -o out/english_create_date target/debug/test_english_date
cargo afl fuzz -i in/bengali_create_date -o out/bengali_create_date target/debug/test_bengali_date
```

## License
This project is licensed under GPL-3.0.