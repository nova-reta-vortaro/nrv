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

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::path::Path;
use std::collections::HashMap;

use rocket::State;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use rand::Rng;

#[derive(Serialize, Deserialize)]
struct Word {
    word: String,
    definition: String,
    #[serde(rename = "see-also")]
    see_also: Vec<String>,
    translations: HashMap<String, Vec<String>>
}

impl Word {
    fn from_file (filename: &str) -> std::io::Result<Word> {
        let complete_path = format!("articles/{}.json", filename);
        let path = Path::new(&complete_path);
        let mut file = File::open(&path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut word : Word = serde_json::from_str(contents.as_str()).unwrap();
        word.definition = markdown::to_html(word.definition.as_str());
        Ok(word)
    }
}

struct Index {
    words: Vec<String>
}

impl Index {
    fn new () -> Index {
        let paths = std::fs::read_dir("articles").unwrap();

        Index {
            words: paths.filter_map(|path| {
                let file = String::from(format!("{}", path.unwrap().path().display()));
                if file.ends_with(".json") {
                    Some(file.replace("articles/", "").replace(".json", ""))
                } else {
                    None
                }
            }).collect()
        }
    }

    fn filter (&self, search: &str) -> Vec<String> {
        let mut res = Vec::new();

        for word in self.words.clone() {
            if word.contains(search) {
                res.push(word.clone())
            }
        }

        res
    }

    fn random (&self) -> String {
        let index = rand::thread_rng().gen_range(0, self.words.len());
        self.words[index].clone()
    }
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

#[derive(FromForm)]
struct SearchQuery {
    demando: Option<String>
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
fn word(vorto: String) -> Result<Template, NotFound<String>> {
    match Word::from_file(&vorto.as_str()) {
        Err(_) => match Word::from_file(&parse_x_notation(vorto).as_str()) {
            Err(_) => Err(NotFound(String::from("Can't find requested file"))),
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

fn parse_x_notation(text: String) -> String {
    text.replace("cx", "ĉ")
        .replace("gx", "ĝ")
        .replace("hx", "ĥ")
        .replace("jx", "ĵ")
        .replace("sx", "ŝ")
}

fn main() {
    rocket::ignite().mount("/", routes![
        static_files,
        index,
        search,
        search_results,
        word,
        random
    ])
    .manage(Index::new())
    .attach(Template::fairing())
    .launch();
}
