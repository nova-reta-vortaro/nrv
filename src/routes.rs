use std;
use std::cell::RefCell;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use serde_json;

use rocket::State;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use daily_article::DailyArticle;
use index::Index;
use utils;
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
fn index(da: State<Mutex<RefCell<DailyArticle>>>, idx: State<Index>) -> std::io::Result<Template> {
    let ref_cell = da.lock().unwrap();
    let da_title = ref_cell.deref().borrow_mut().get(&idx);

    match Word::from_file(da_title.as_str()) {
        Err(why) => { // refresh the daily word if the current one is broken
            ref_cell.deref().borrow_mut().refresh(&idx);
            Err(why)
        },
        Ok(daily_article) => {
            let mut def = daily_article.meanings[0].definition.clone();
            let limit = match def.len() > 100 {
                true => 100,
                false => def.len()
            };
            def.split_off(limit);
            Ok(Template::render("index", &json!({
                "selected": "/",
                "daily_article": {
                    "title": da_title,
                    "preview": def
                }
            })))
        }
    }
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
        "results": index.filter(&utils::parse_x_notation(query.demando.unwrap_or("".to_string())))
    }))
}

#[get("/vorto/<vorto>")]
fn word(vorto: String) -> std::io::Result<Template> {
    utils::find_word(vorto).map(|data| {
        Template::render("word", &serde_json::to_value(&data).unwrap())
    })
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
