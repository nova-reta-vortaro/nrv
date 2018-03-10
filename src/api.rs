use utils;

use std::io::Result;
use std::error::Error;
use serde::Serialize;
use rocket_contrib::Json;

fn to_json<T> (data: Result<T>)-> Json where T: Serialize {
    match data {
        Err(why) => Json(json!({
            "error": why.description()
        })),
        Ok(x) => Json(json!(x))
    }
}

#[get("/<vorto>")]
fn word(vorto: String) -> Json {
    let w = utils::find_word(vorto);
    to_json(w)
}