use crate::index::Index;
use crate::utils;

use rocket::get;
use rocket::State;
use serde::Serialize;
use serde_json::json;
use std::cell::RefCell;
use std::io::Result;
use std::sync::Mutex;

pub struct Json(serde_json::Value);

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Json {
    fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'o> {
        let json = serde_json::to_string(&self.0).unwrap();
        rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .ok()
    }
}

fn to_json<T>(data: Result<T>) -> Json
where
    T: Serialize,
{
    match data {
        Err(why) => Json(json!({
            "error": why.to_string()
        })),
        Ok(x) => Json(json!(x)),
    }
}

#[get("/vorto/<vorto>")]
pub fn word(vorto: String) -> Json {
    let w = utils::find_word(vorto);
    to_json(w)
}

#[get("/sercxu?<demando>")]
pub fn search_results(demando: String, index: &State<Mutex<RefCell<Index>>>) -> Json {
    let index = index.lock().unwrap();
    let index = index.borrow();
    Json(json!({
        "results": index.filter(&utils::parse_x_notation(demando))
    }))
}

#[get("/hazarda")]
pub fn random(index: &State<Mutex<RefCell<Index>>>) -> Json {
    let index = index.lock().unwrap();
    let index = index.borrow();
    let article_name = index.random();
    to_json(utils::find_word(article_name))
}
