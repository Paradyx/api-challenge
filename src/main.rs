use actix_web::{HttpResponse, web, HttpServer, App};
use challenge::{level, success};

async fn challenge(path: web::Path<usize>) -> HttpResponse {
    let page_no = path.into_inner();

    let page = match page_no {
        0..=9 => level(1, page_no), //sane
        10..=19 => level(2, page_no), // random
        20..=29 => level(3, page_no), // drops
        30..=39 => level(4, page_no), // errors
        40..=49 => level(5, page_no), // invalid timestamps
        50..=59 => level(6, page_no), // large picture
        60..=69 => level(7, page_no), // large page_size
        _ => success()
    };

    page.into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/usage/{pageNo}", web::get().to(challenge))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

