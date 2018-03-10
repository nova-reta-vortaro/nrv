use utils;
use index::Index;

use std::io::Result;
use std::error::Error;
use serde::Serialize;
use rocket::State;
use rocket_contrib::Json;

fn to_json<T> (data: Result<T>)-> Json where T: Serialize {
    match data {
        Err(why) => Json(json!({
            "error": why.description()
        })),
        Ok(x) => Json(json!(x))
    }
}

#[get("/vorto/<vorto>")]
fn word(vorto: String) -> Json {
    let w = utils::find_word(vorto);
    to_json(w)
}

#[get("/sercxu?<query>")]
fn search_results(query: utils::SearchQuery, index: State<Index>) -> Json {
    Json(json!({
        "results": index.filter(&utils::parse_x_notation(query.demando.unwrap_or("".to_string())))
    }))
}