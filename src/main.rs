#![feature(decl_macro, proc_macro_hygiene)]

mod api;
mod daily_article;
mod index;
mod word;
mod utils;
mod routes;

use std::cell::RefCell;
use std::sync::Mutex;

use rocket::{routes, catchers};

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::static_files,
            routes::index,
            routes::search,
            routes::word,
            routes::random,
            routes::import,
            routes::send_import
        ])
        .mount("/api", routes![
            api::word,
            api::search_results,
            api::random
        ])
        .register(catchers![
            routes::not_found,
            routes::server_error
        ])
        .manage(Mutex::new(RefCell::new(index::Index::new())))
        .manage(Mutex::new(RefCell::new(daily_article::DailyArticle::new())))
        .launch();
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
