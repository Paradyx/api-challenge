use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::Deref;

pub struct Cpu(&'static str);

impl Distribution<Cpu> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cpu {
        Cpu(CPUS[rng.gen_range(0, 24)])
    }
}

impl Deref for Cpu {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const CPUS: [&str; 24] = [
    "Intel(R) Core(TM) i7-4770 CPU @ 3.40GHz",
    "Intel(R) Core(TM) i5-5200U CPU @ 2.20GHz",
    "Intel(R) Celeron(R) CPU N2807 @ 1.58GHz",
    "Intel(R) Core(TM) i7-4790T CPU @ 2.70GHz",
    "Intel(R) Core(TM) i3-8300 CPU @ 3.70GHz",
    "Intel(R) Xeon(R) CPU E5-2640 0 @ 2.50GHz",
    "Intel(R) Core(TM) i7-4790S CPU @ 3.20GHz",
    "Pentium(R) Dual-Core CPU E6000 @ 3.46GHz",
    "AMD Ryzen Threadripper 3970X 32-Core Processor",
    "AMD Ryzen 5 3600X 6-Core Processor",
    "AMD Ryzen 5 PRO 2500U w/ Radeon Vega Mobile Gfx",
    "AMD Ryzen 3 1300X Quad-Core Processor",
    "AMD Ryzen 7 PRO 3700U w/ Radeon Vega Mobile Gfx",
    "Intel(R) Xeon(R) CPU E3-1535M v5 @ 2.90GHz",
    "Intel(R) Core(TM) i7-5950HQ CPU @ 2.90GHz",
    "Intel(R) Xeon(R) CPU E3-1230 v6 @ 3.50GHz",
    "Intel(R) Xeon(R) CPU E5-4650 0 @ 2.70GHz",
    "Intel(R) Core(TM) i5-1035G1 CPU @ 1.00GHz",
    "AMD Ryzen 3 4300U with Radeon Graphics",
    "Intel(R) Core(TM) i5-8257U CPU @ 1.40GHz",
    "Intel(R) Xeon(R) CPU E5-1660 0 @ 3.30GHz",
    "Intel(R) Xeon(R) CPU E3-1275 v6 @ 3.80GHz",
    "AMD Ryzen 5 PRO 2400G with Radeon Vega Graphics",
    "AMD Athlon(tm) II X4 640 Processor",
];

