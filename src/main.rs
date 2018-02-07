#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
extern crate markdown;

use std::error::Error;
use rocket_contrib::Template;
use rocket::response::status::NotFound;
use std::path::PathBuf;
use std::path::Path;
use rocket::response::NamedFile;

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.to_str().unwrap_or("error"))))
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &json!({
        "selected": "/"
    }))
}

#[get("/sercxu")]
fn search() -> Template {
    Template::render("search", &json!({
        "selected": "/sercxu",
        "query": "",
        "results": []
    }))
}

#[derive(FromForm)]
struct SearchQuery {
    demando: Option<String>
}

#[get("/sercxu?<query>")]
fn search_results(query: SearchQuery) -> Template {
    Template::render("search", &json!({
        "selected": "/sercxu",
        "query": query.demando,
        "results": [
            "test"
        ]
    }))
}

#[get("/vorto/<vorto>")]
fn word(vorto: String) -> Result<Template, NotFound<String>> {
    let path = Path::new("articles/").join(format!("{}.md", vorto));
    match markdown::file_to_html(&path) {
        Err(why) => Err(NotFound(String::from(why.description()))),
        Ok(html) => {
            Ok(Template::render("word", &json!({
                "title": vorto,
                "content": html,
            })))
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![
        static_files,
        index,
        search,
        search_results,
        word
    ])
    .attach(Template::fairing())
    .launch();
}
