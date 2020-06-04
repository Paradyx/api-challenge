use rand::distributions::{Standard, Distribution};
use rand::Rng;
use std::ops::Deref;

pub struct OperatingSystem(&'static str);

impl Distribution<OperatingSystem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OperatingSystem{
        OperatingSystem(OPERATING_SYSTEMS[rng.gen_range(0, 13)])
    }
}

impl Deref for OperatingSystem{
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const OPERATING_SYSTEMS: [&str; 13] = [
    "Arch Linux; 5.6.14-arch1-1",
    "Ubuntu 16.04.6; 4.4.0-13.29~14.04.1; ", //xenial
    "Ubuntu 18.04.4; 4.15.0-102.103", //bionic
    "Ubuntu 19.10; 5.3.0-56.50", //eoan
    "Ubuntu 20.04; 5.4.0-34.38", //focal
    "Fedora release 31 (Thirty One); 5.3.7-301 ",
    "Red Hat Enterprise Linux release 8.0 Beta (Ootpa); 4.18.0-80",
    "CentOS Linux release 7.6.1810 (Core); 3.10.0-957",
    "Microsoft Windows XP Professional; 5.1.2600 Service Pack 2 Build 2600",
    "Microsoft Windows 7 Ultimate; 6.1.7600 N/A Build 7600",
    "Microsoft Windows 7 Enterprise;  6.1.7601 Service Pack 1 Build 7601",
    "Microsoft Windows Server 2008 R2 Enterprise; 6.1.7600 N/A Build 7600",
    "Microsoft Windows 10 Pro; 10.0.18363 N/A Build 18363",
];