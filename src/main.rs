mod api;
mod daily_article;
mod index;
mod routes;
mod utils;
mod word;

use std::cell::RefCell;
use std::sync::Mutex;

use rocket::{catchers, routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                routes::static_files,
                routes::index,
                routes::search,
                routes::word,
                routes::random,
                routes::import,
                routes::send_import
            ],
        )
        .mount("/api", routes![api::word, api::search_results, api::random])
        .register("/", catchers![routes::not_found, routes::server_error])
        .manage(Mutex::new(RefCell::new(index::Index::new())))
        .manage(Mutex::new(RefCell::new(daily_article::DailyArticle::new())))
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
