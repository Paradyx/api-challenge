//! Procedural generation of dummy usage data.

mod cpu;
mod dates;
mod email;
mod hostname;
mod ip;
mod names;
mod os;
mod usages;
mod image;

pub use usages::Usage;
pub use image::IMAGE;
use rand_xorshift::XorShiftRng;
use serde_json::{Value, Map};
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use chrono::DateTime;

trait DeepShuffle {
    fn deep_shuffle<R: Rng>(self, prng: &mut R) -> Self;
}

impl DeepShuffle for Value {
    fn deep_shuffle<R: Rng>(self, prng: &mut R) -> Value {
        match self {
            Value::Object(map) => {
                let mut list: Vec<(String, Value)> = map.into_iter().collect();
                list.shuffle(prng);

                let mut shuffled: Map<String, Value> = Map::new();
                for (key, value) in list {
                    shuffled.insert(key.clone(), value.deep_shuffle(prng));
                }
                Value::Object(shuffled)
            }
            _ => self
        }
    }
}

/// Iterator that generates an endless amount of usage data, base on a [`XorShiftRng`].
pub struct UsageGenerator {
    prng: XorShiftRng,
    lvl: u64,
}

impl UsageGenerator {
    pub fn new(lvl: u64, page_no: usize) -> UsageGenerator {
        UsageGenerator {
            prng: XorShiftRng::seed_from_u64(((0xA as u64) << (16 + lvl)) + (page_no as u64)),
            lvl,
        }
    }

    fn random_drop(&mut self, value: &mut Value ) {
        // randomly drop a name field
        if self.prng.gen_bool(0.1) {
            match self.prng.gen_range(0, 2) {
                0 => value["account"]["first_name"].take(),
                1 => value["account"]["last_name"].take(),
                _ => value["account"]["full_name"].take(),
            };
        }
        // randomly empty the email
        if self.prng.gen_bool(0.1) {
            value["account"]["email"] = Value::String("".to_string());
        }

        // randomly drop device information
        if self.prng.gen_bool(0.1) {
            match self.prng.gen_range(0, 4) {
                0 => value["device"]["cpu"].take(),
                1 => value["device"]["cpu"].take(),
                2 => value["device"]["cpu"].take(),
                _ => value["device"]["cpu"].take(),
            };
        }

        // randomly drop usage time
        if self.prng.gen_bool(0.05) {
            value["usage_time"].take();
        }
    }

    fn vary_timestamp(&mut self, value: &mut Value) {
        let date = DateTime::parse_from_rfc3339(value["usage_date"].as_str().unwrap()).unwrap();
        if self.prng.gen_bool(0.15) {// use epoch
            value["usage_date"] = Value::from(date.timestamp());
        } else if self.prng.gen_bool(0.15) {
            value["usage_date"] = Value::from(date.to_rfc2822());
        }
    }

    fn add_image(&mut self, value: &mut Value) {
        if self.prng.gen_bool(0.4) {
            value["account"]["profile_picture"] = Value::from(IMAGE);
        }
    }
}

impl Iterator for UsageGenerator {
    type Item = Value;

    /// Procedurally generate the next item
    fn next(&mut self) -> Option<Self::Item> {
        let mut profile = Value::from(self.prng.gen::<Usage>());

        // lvl2 shuffle json
        if self.lvl >= 2 {
            profile = profile.deep_shuffle(&mut self.prng)
        }

        // lvl3 randomly drop fields
        if self.lvl >= 3 {
            self.random_drop(&mut profile);
        }

        // lvl4 error codes is implemented somewhere else

        // lvl5 inconsisten timestamps
        if self.lvl >= 5 {
            self.vary_timestamp(&mut profile);
        }

        // lvl6 add images
        if self.lvl >= 6{
            self.add_image(&mut profile);
        }

        Some(profile)
    }
}
