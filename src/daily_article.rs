use time;
use index::Index;

///
/// Get a new article every day, to feature it on the home page.
/// 
pub struct DailyArticle {
    pub last_gen_time: time::Tm,
    pub article: Option<String>
}

impl DailyArticle {
    pub fn new() -> DailyArticle {
        DailyArticle {
            last_gen_time: time::now(),
            article: None
        }
    }

    pub fn refresh(&mut self, index: &Index) {
        self.last_gen_time = time::now();
        self.article = Some(index.random());
    }

    pub fn get(&mut self, index: &Index) -> String {
        let now = time::now();
        if now - self.last_gen_time > time::Duration::days(1) || self.article.is_none() {
            self.refresh(index);
        }
        self.article.clone().unwrap_or(String::from(""))
    }
}
