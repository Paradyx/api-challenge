use rand::distributions::{Standard, Distribution};
use rand::Rng;

/// Typical email patterns taken from https://www.lifewire.com/how-to-guess-email-addresses-1170885
pub enum EmailPattern {
    /// flast@example.com
    FLAST,
    /// first.last@example.com
    DOTSEPARATED,
    /// firstlast@example.com
    COLLATED,
    /// last@example.com
    LAST,
    /// first_last@example.com
    SNAKE,
    /// f_last@example.com
    FLASTSNAKE,
    /// firstl@example.com
    FIRSTL,
}

impl Distribution<EmailPattern> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EmailPattern {
        match rng.gen_range(0, 7) {
            0 => EmailPattern::FLAST,
            1 => EmailPattern::DOTSEPARATED,
            2 => EmailPattern::COLLATED,
            3 => EmailPattern::LAST,
            4 => EmailPattern::SNAKE,
            5 => EmailPattern::FLASTSNAKE,
            _ => EmailPattern::FIRSTL,
        }
    }
}

impl EmailPattern {
    pub fn for_name(&self, first: &String, last: &String, domain: &String) -> String {
        return match self {
            EmailPattern::FLAST => format!("{}{}@{}", first.chars().next().unwrap(), last, domain),
            EmailPattern::DOTSEPARATED => format!("{}.{}@{}", first, last, domain),
            EmailPattern::COLLATED => format!("{}{}@{}", first, last, domain),
            EmailPattern::LAST => format!("{}@{}", last, domain),
            EmailPattern::SNAKE => format!("{}_{}@{}", first, last, domain),
            EmailPattern::FLASTSNAKE => format!("{}_{}@{}", first.chars().next().unwrap(), last, domain),
            EmailPattern::FIRSTL => format!("{}{}@{}", first, last.chars().next().unwrap(), domain),
        }
    }
}
