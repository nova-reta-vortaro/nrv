use crate::utils;
use crate::index::Index;

use std::io::Result;
use std::error::Error;
use serde::Serialize;
use rocket::get;
use rocket::State;
use serde_json::json;

pub struct Json(serde_json::Value);

impl<'r> rocket::response::Responder<'r> for Json {
    fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'r> {
        rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(std::io::Cursor::new(serde_json::to_string(&self.0).unwrap()))
            .ok()
    }
}

fn to_json<T>(data: Result<T>)-> Json where T: Serialize {
    match data {
        Err(why) => Json(json!({
            "error": why.description()
        })),
        Ok(x) => Json(json!(x))
    }
}

#[get("/vorto/<vorto>")]
pub fn word(vorto: String) -> Json {
    let w = utils::find_word(vorto);
    to_json(w)
}

#[get("/sercxu?<demando>")]
pub fn search_results(demando: String, index: State<Index>) -> Json {
    Json(json!({
        "results": index.filter(&utils::parse_x_notation(demando))
    }))
}

#[get("/hazarda")]
pub fn random(index: State<Index>) -> Json {
    let article_name = index.random();
    to_json(utils::find_word(article_name))
}
