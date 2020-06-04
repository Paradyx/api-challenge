//! This application exposes an API on `localhost:8000` which is filled with dummy data. The data
//! is procedurally generated and streamed out. Hence, the memory footprint of the application is
//! rather small.
//!
//! The API is paginated. With increasing page number, the difficulty to use the API increases.

#[macro_use]
extern crate serde_json;

use actix_web::HttpResponse;
use crate::procedural::UsageGenerator;
use actix_web::http::StatusCode;
use std::thread::sleep;
use std::time::Duration;
use bytes::Bytes;
use rand::Rng;
use futures::stream::{self};
use core::iter;

pub mod procedural;

/// Number of items in a normal page.
const PAGE_SIZE: usize = 10;

/// Number of items in a large page used in higher level.
const HUGE_PAGE_SIZE: usize = 1000;

/// Procedurally generate a response for the API. The response depends on the level number and the
/// and the requested pagenumber. The higher the level, the more the response is modified to make
/// the API harder to use.
/// The page number is used to procedurally generate the initial usage data, so that the API looks
/// realistic.
///
/// The data is generated on the fly and written as stream to the response. Hence it is not generated
/// ahead of time and the memory footprint is small.
///
/// # Difficulty levels
/// 1. sane and well structured JSON;
/// 2. randomized order of JSON properties;
/// 3. some fields are procedurally `null`ed and others are replaced by the empty string `""`;
/// 4. with p = 0.25, the response is delayed up to 20 seconds; with p = 0.25, the response contains an error;
/// 5. some timestamps are procedurally replaced by RFC 2822 or Unix timestamps instead of RFC 3339;
/// 6. some usages contain a large image, that artificially bloats the data;
/// 7. the page size is drastically increased (~ 1GB).
/// 9. a success message is printed
///
/// # Example
///
/// ```
/// use challenge::level;
/// let response = level(1, 8);
/// ```
/// The line above generates an HttpResponse with difficulty 1 (easy and sane JSON data).
///
pub fn level(difficulty: u64, page_no: usize) -> HttpResponse {
    if difficulty >= 9 {
        return success()
    }

    let mut rng = rand::thread_rng();

    // lvl 5 random wait
    if difficulty >= 4 && difficulty <= 5 { // Don't wait when images are part of the response
        // random sleep
        if rng.gen_bool(0.25) {
            sleep(Duration::from_millis(rng.gen_range(1500, 20000)))
        };
    }

    // lvl 4 random errors
    if difficulty >= 4 {
        if rng.gen_bool(0.25) {
            return match rng.gen_range(0, 5) {
                0 => HttpResponse::InternalServerError().finish(),
                1 => HttpResponse::TooManyRequests().finish(),
                2 => HttpResponse::ServiceUnavailable().finish(),
                3 => HttpResponse::InsufficientStorage().finish(),
                _ => HttpResponse::GatewayTimeout().finish(),
            };
        }
    }

    // lvl7 large page size
    let page_size = if difficulty >= 7 {
        HUGE_PAGE_SIZE
    } else {
        PAGE_SIZE
    };

    let before = format!("{{\"level\": {}, \"usages\": [", difficulty);
    let after = "]}".to_string();

    let it = iter::once::<Result<Bytes, ()>>(Ok(Bytes::copy_from_slice(before.as_bytes())))
        .chain(UsageGenerator::new(difficulty, page_no)
            .take(page_size)
            .enumerate()
            .map(move |(i, profile)| {
                if i < page_size - 1 {
                    format!("{},", profile.to_string())
                } else {
                    profile.to_string()
                }
            }).map(|raw| Ok(Bytes::copy_from_slice(raw.as_bytes())))
        ).chain(iter::once(Ok(Bytes::from(after))));

    HttpResponse::Ok().streaming(stream::iter(it))
}

/// Produce a response that indicates the end of the challenge.
pub fn success() -> HttpResponse {
    return HttpResponse::build(StatusCode::from_u16(418).unwrap()).body("Congratulation you finished the challenge"); // Im a Teapot
}