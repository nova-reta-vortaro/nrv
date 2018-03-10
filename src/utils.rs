use std::io::Result;
use word::Word;

pub fn parse_x_notation(text: String) -> String {
    text.replace("cx", "ĉ")
        .replace("gx", "ĝ")
        .replace("hx", "ĥ")
        .replace("jx", "ĵ")
        .replace("sx", "ŝ")
        .replace("ux", "ŭ")
}

pub fn find_word(word: String) -> Result<Word> {
    match Word::from_file(&word.as_str()) {
        Err(_) => match Word::from_file(&parse_x_notation(word).as_str()) {
            Err(why) => Err(why),
            Ok(data) => Ok(data)
        },
        Ok(data) => Ok(data)
    }
}

#[derive(FromForm)]
pub struct SearchQuery {
    pub demando: Option<String>
}