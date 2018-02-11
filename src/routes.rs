use std;
use std::path::Path;
use std::path::PathBuf;

use serde_json;

use rocket::State;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use index::Index;
use utils::parse_x_notation;
use word::Word;

#[derive(FromForm)]
struct SearchQuery {
    demando: Option<String>
}

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

#[get("/sercxu?<query>")]
fn search_results(query: SearchQuery, index: State<Index>) -> Template {
    Template::render("search", &json!({
        "selected": "/sercxu",
        "query": query.demando,
        "results": index.filter(&parse_x_notation(query.demando.unwrap_or("".to_string())))
    }))
}

#[get("/vorto/<vorto>")]
fn word(vorto: String) -> std::io::Result<Template> {
    match Word::from_file(&vorto.as_str()) {
        Err(_) => match Word::from_file(&parse_x_notation(vorto).as_str()) {
            Err(why) => Err(why),
            Ok(data) => Ok(Template::render("word", &serde_json::to_value(&data).unwrap()))
        },
        Ok(data) => Ok(Template::render("word", &serde_json::to_value(&data).unwrap()))
    }
}

#[get("/hazarda")]
fn random(index: State<Index>) -> Redirect {
    let article_name = index.random();
    Redirect::to(&format!("/vorto/{}", article_name))
}

#[error(500)]
fn server_error() -> Template {
    Template::render("errors/500", &json!({}))
}

#[error(404)]
fn not_found() -> Template {
    Template::render("errors/404", &json!({}))
}
