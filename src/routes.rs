use std;
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::get;
use rocket::catch;
use rocket::uri;
use rocket::State;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

use crate::daily_article::DailyArticle;
use crate::index::Index;
use crate::utils;
use crate::word::Word;

pub struct Template(Vec<u8>);

impl<'r> rocket::response::Responder<'r> for Template {
    fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'r> {
        rocket::response::Response::build()
            .header(rocket::http::ContentType::HTML)
            .sized_body(std::io::Cursor::new(self.0))
            .ok()
    }
}

macro_rules! render {
    ($page:ident ( $( $param:expr ),* ) ) => {
        {
            use crate::templates;

            let mut res = vec![];
            templates::$page(
                &mut res,
                $(
                    $param
                ),*
            ).unwrap();
            Template(res)
        }
    };
    ($m:ident :: $page:ident ( $( $param:expr ),* ) ) => {
        {
            use crate::templates;

            let mut res = vec![];
            templates::$m::$page(
                &mut res,
                $(
                    $param
                ),*
            ).unwrap();
            Template(res)
        }
    }
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.to_str().unwrap_or("error"))))
}

#[get("/")]
pub fn index(da: State<Mutex<RefCell<DailyArticle>>>, index: State<Mutex<RefCell<Index>>>) -> std::io::Result<Template> {
    let rc = index.lock().unwrap();
    let idx = rc.borrow();

    let ref_cell = da.lock().unwrap();
    let da_title = ref_cell.borrow_mut().get(&idx);

    match Word::from_file(da_title.as_str()) {
        Err(why) => { // refresh the daily word if the current one is broken
            ref_cell.borrow_mut().refresh(&idx);
            Err(why)
        },
        Ok(daily_article) => {
            Ok(render!(index(daily_article)))
        }
    }
}

#[get("/sercxu?<demando>")]
pub fn search(demando: Option<String>, index: State<Mutex<RefCell<Index>>>) -> Template {
    if let Some(d) = demando {
        let rc = index.lock().unwrap();
        let idx = rc.borrow();
        render!(search(d.clone(), idx.filter(&utils::parse_x_notation(d))))
    } else {
        render!(search(String::new(), vec![]))
    }
}

#[get("/vorto/<vorto>")]
pub fn word(vorto: String) -> std::io::Result<Template> {
    utils::find_word(vorto).map(|data| {
        render!(word(data))
    })
}

#[get("/hazarda")]
pub fn random(index: State<Mutex<RefCell<Index>>>) -> Redirect {
    let rc = index.lock().unwrap();
    let idx = rc.borrow();
    let article_name = idx.random();
    Redirect::to(uri!(word: vorto = article_name))
}

#[get("/importo")]
pub fn import() -> Template {
    render!(import())
}

#[get("/importo?<demando>")]
pub fn send_import(demando: String, index: State<Mutex<RefCell<Index>>>) -> Redirect {
    let rc = index.lock().unwrap();
    let mut idx = rc.borrow_mut();
    idx.import(demando);
    Redirect::to(uri!(index))
}

#[catch(500)]
pub fn server_error() -> Template {
    render!(errors::server_error())
}

#[catch(404)]
pub fn not_found() -> Template {
    render!(errors::not_found())
}
