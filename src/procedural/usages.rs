use rand::{Rng};
use rand::distributions::{Distribution, Standard};
use uuid::Uuid;
use crate::procedural::names::{FirstName, LastName};
use crate::procedural::email::EmailPattern;
use crate::procedural::os::OperatingSystem;
use crate::procedural::cpu::Cpu;
use crate::procedural::ip::Ip;
use crate::procedural::dates::UsageDates;
use chrono::{Utc, DateTime};
use crate::procedural::hostname::Hostname;
use serde_json::Value;

/// The data object at the core of this API is a single Usage, which represents the use of a
/// fantasy application by a single user. The usage is filled with data which make it more realistic.
/// Look at the fields description for more details about the data.
///
/// # Procedural generation
/// Thanks to the Distribution trait, a Usage can be procedurally generated from a (P)RNG.
///
/// ```
/// use rand::Rng;
/// use challenge::procedural::Usage;
///
/// let mut rng = rand::thread_rng();
/// let usage = rng.gen::<Usage>();
/// ```
pub struct Usage {
    /// A UUID which is unique for usage. Collisions are possible, but very unlikely.
    pub uuid: String,
    /// The first name of the user, which used the fantasy application.
    pub first_name: String,
    /// The last name of the user, which used the fantasy application.
    pub last_name: String,
    /// The full name of the user, which used the fantasy application. It always is the collation
    /// of [first_name] and [last_name].
    pub full_name: String,
    /// The email address of the user, which used the fantasy application. The address is derived
    /// from the name of the user.
    pub email: String,
    /// A fantasy hostname of the device, from which the usage occurred.
    pub hostname: String,
    /// A fantasy IP address of the device, from which the usage occurred.
    pub ip_address: String,
    /// The operating system of the device, from which the usage occurred.
    pub operating_system: String,
    /// The CPU description for the CPU of the device, from which the usage occurred.
    pub cpu: String,
    /// Flag, that indicates, if the account is a demo account. Hence it also hints if the usage
    /// is billable or not.
    pub is_demo: bool,
    /// Date at which the usage took place.
    pub usage_date: DateTime<Utc>,
    /// Date at which the user account was created.
    pub created_on: DateTime<Utc>,
    /// The total time of the usage.
    pub usage_time: u64,
}


impl Distribution<Usage> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Usage {
        let first_name = rng.gen::<FirstName>().to_string();
        let last_name = rng.gen::<LastName>().to_string();
        let full_name = format!("{} {}", first_name, last_name);
        let domain = "example.com".to_string();
        let pattern: EmailPattern = rng.gen();
        let email = pattern.for_name(&first_name, &last_name, &domain);
        let ip_address = rng.gen::<Ip>().clone();
        let hostname = rng.gen::<Hostname>().to_string();

        let mut buf = [b'!'; 40];
        let uuid = Uuid::from_u128(rng.gen::<u128>())
            .to_hyphenated()
            .encode_lower(&mut buf)
            .to_string();

        let operating_system = rng.gen::<OperatingSystem>().to_string();
        let cpu = String::from(*rng.gen::<Cpu>());

        let dates = rng.gen::<UsageDates>();
        let is_demo = rng.gen::<bool>();
        let person = Usage {
            uuid,
            first_name,
            last_name,
            full_name,
            email,
            hostname,
            ip_address,
            operating_system,
            cpu,
            is_demo,
            created_on: dates.created_on,
            usage_date: dates.usage_date,
            usage_time: dates.usage_time,
        };

        person
    }
}

/// Transform into a sane value
impl From<Usage> for Value {
    fn from(profile : Usage) -> Self {
            json!({
            "uuid": profile.uuid,
            "account": {
                "first_name": profile.first_name,
                "last_name": profile.last_name,
                "full_name": profile.full_name,
                "email": profile.email,
                "created_on": profile.created_on.to_rfc3339(),
            },
            "device": {
                "hostname": profile.hostname,
                "ip_address": profile.ip_address,
                "operating_system": profile.operating_system,
                "cpu": profile.cpu,
            },
            "usage_date": profile.usage_date.to_rfc3339(),
            "usage_time": profile.usage_time,
        })
    }
}
