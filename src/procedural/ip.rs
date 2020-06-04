use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::Deref;

pub struct Ip(String);

impl Distribution<Ip> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ip {
        let subnet = rng.gen_range(0, 128);
        let addr = rng.gen_range(1, 255);
        let ip = format!("192.168.{}.{}", subnet, addr);
        Ip(ip)
    }
}

impl Deref for Ip {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
