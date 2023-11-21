use std;
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::catch;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::uri;
use rocket::State;

use crate::daily_article::DailyArticle;
use crate::index::Index;
use crate::templates;
use crate::utils;
use crate::word::Word;

pub struct Template(Vec<u8>);

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Template {
    fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'o> {
        rocket::response::Response::build()
            .header(rocket::http::ContentType::HTML)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

impl Template {
    fn render(f: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>) -> Self {
        let mut res = vec![];
        f(&mut res).unwrap();
        Template(res)
    }
}

#[get("/static/<file..>")]
pub async fn static_files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path)
        .await
        .map_err(|_| NotFound(format!("Bad path: {}", path.to_str().unwrap_or("error"))))
}

#[get("/")]
pub fn index(
    da: &State<Mutex<RefCell<DailyArticle>>>,
    index: &State<Mutex<RefCell<Index>>>,
) -> std::io::Result<Template> {
    let rc = index.lock().unwrap();
    let idx = rc.borrow();

    let ref_cell = da.lock().unwrap();
    let da_title = ref_cell.borrow_mut().get(&idx);

    match Word::from_file(da_title.as_str()) {
        Err(why) => {
            // refresh the daily word if the current one is broken
            ref_cell.borrow_mut().refresh(&idx);
            Err(why)
        }
        Ok(daily_article) => Ok(Template::render(|out| {
            templates::index_html(out, daily_article)
        })),
    }
}

#[get("/sercxu?<demando>")]
pub fn search(demando: Option<String>, index: &State<Mutex<RefCell<Index>>>) -> Template {
    if let Some(d) = demando {
        let rc = index.lock().unwrap();
        let idx = rc.borrow();
        Template::render(|out| {
            templates::search_html(out, d.clone(), idx.filter(&utils::parse_x_notation(d)))
        })
    } else {
        Template::render(|out| templates::search_html(out, String::new(), vec![]))
    }
}

#[get("/vorto/<vorto>")]
pub fn word(vorto: String) -> std::io::Result<Template> {
    utils::find_word(vorto).map(|data| Template::render(|out| templates::word_html(out, data)))
}

#[get("/hazarda")]
pub fn random(index: &State<Mutex<RefCell<Index>>>) -> Redirect {
    let rc = index.lock().unwrap();
    let idx = rc.borrow();
    let article_name = idx.random();
    Redirect::to(uri!(word(vorto = article_name)))
}

#[get("/importo")]
pub fn import() -> Template {
    Template::render(|out| templates::import_html(out))
}

#[get("/importo?<demando>")]
pub fn send_import(demando: String, index: &State<Mutex<RefCell<Index>>>) -> Redirect {
    let rc = index.lock().unwrap();
    let mut idx = rc.borrow_mut();
    idx.import(demando);
    Redirect::to(uri!(index))
}

#[catch(500)]
pub fn server_error() -> Template {
    Template::render(|out| templates::errors::server_error_html(out))
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render(|out| templates::errors::not_found_html(out))
}
