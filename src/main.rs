#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate markdown;
extern crate rand;
extern crate time;

mod api;
mod daily_article;
mod index;
mod word;
mod utils;
mod routes;

use std::cell::RefCell;
use std::sync::Mutex;

use rocket_contrib::Template;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::static_files,
            routes::index,
            routes::search,
            routes::search_results,
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
        .catch(errors![
            routes::not_found,
            routes::server_error
        ])
        .manage(Mutex::new(RefCell::new(index::Index::new())))
        .manage(Mutex::new(RefCell::new(daily_article::DailyArticle::new())))
        .attach(Template::fairing())
        .launch();
}
