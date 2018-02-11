#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate markdown;
extern crate rand;

mod index;
mod word;
mod utils;
mod routes;

use rocket_contrib::Template;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::static_files,
            routes::index,
            routes::search,
            routes::search_results,
            routes::word,
            routes::random
        ])
        .catch(errors![
            routes::not_found,
            routes::server_error
        ])
        .manage(index::Index::new())
        .attach(Template::fairing())
        .launch();
}
