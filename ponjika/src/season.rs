pub enum BengaliSeasons {
    Grishmo,
    Borsha,
    Sharat,
    Hemonto,
    Sheet,
    Bashonto,
}

impl BengaliSeasons {
    pub fn as_str(&self) -> &str {
        match self {
            BengaliSeasons::Grishmo => "গ্রীষ্ম",
            BengaliSeasons::Borsha => "বর্ষা",
            BengaliSeasons::Sharat => "শরৎ",
            BengaliSeasons::Hemonto => "হেমন্ত",
            BengaliSeasons::Sheet => "শীত",
            BengaliSeasons::Bashonto => "বসন্ত",
        }
    }

    pub fn from_month(month: u8) -> BengaliSeasons {
        match month {
            1 | 2 => BengaliSeasons::Grishmo,
            3 | 4 => BengaliSeasons::Borsha,
            5 | 6 => BengaliSeasons::Sharat,
            7 | 8 => BengaliSeasons::Hemonto,
            9 | 10 => BengaliSeasons::Sheet,
            11 | 12 => BengaliSeasons::Bashonto,
            _ => BengaliSeasons::Sheet,
        }
    }
}
