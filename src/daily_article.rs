use chrono::prelude::*;
use crate::index::Index;

///
/// Get a new article every day, to feature it on the home page.
///
pub struct DailyArticle {
    pub last_gen_time: chrono::DateTime<Utc>,
    pub article: Option<String>,
}

impl DailyArticle {
    pub fn new() -> DailyArticle {
        DailyArticle {
            last_gen_time: Utc::now(),
            article: None,
        }
    }

    pub fn refresh(&mut self, index: &Index) {
        self.last_gen_time = Utc::now();
        self.article = Some(index.random());
    }

    pub fn get(&mut self, index: &Index) -> String {
        let now = Utc::now();
        if now - self.last_gen_time > chrono::Duration::days(1) || self.article.is_none() {
            self.refresh(index);
        }
        self.article.clone().unwrap_or(String::from(""))
    }
}
