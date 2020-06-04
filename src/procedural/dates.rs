use chrono::{DateTime, Utc, TimeZone, Duration};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub struct UsageDates {
    pub created_on: DateTime<Utc>,
    pub usage_date: DateTime<Utc>,
    pub usage_time: u64,
}

impl Distribution<UsageDates> for Standard {
    fn sample<R>(&self, rng: &mut R) -> UsageDates where R: Rng + ?Sized {
        let earliest = Utc.ymd(2019, 1, 1).and_hms(0, 0 ,0);
        let latest_created = Utc.ymd(2020, 05, 15).and_hms(0, 0, 0);
        let latest_usage = Utc.ymd(2020,05, 28).and_hms(0, 0, 0);

        let create_period = latest_created.signed_duration_since(earliest);
        let created_millis = create_period.num_milliseconds();
        let created_on = earliest +Duration::milliseconds(rng.gen_range(0, created_millis));

        let usage_period = latest_usage.signed_duration_since(created_on);
        let usage_millis = usage_period.num_milliseconds();
        let usage_date = created_on + Duration::milliseconds(rng.gen_range(0, usage_millis));

        let usage_time: u64 = rng.gen_range(3000, 86400000);
        UsageDates {
            created_on,
            usage_date,
            usage_time,
        }
    }
}
