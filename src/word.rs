use std;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use markdown;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Meaning {
    pub usage: String,
    pub definition: String,
    pub examples: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub meanings: Vec<Meaning>,
    pub translations: HashMap<String, Vec<String>>,
    pub related: Vec<String>,
    pub bibliography: Vec<String>
}

impl Word {
    pub fn from_file (filename: &str) -> std::io::Result<Word> {
        let complete_path = format!("articles/{}.json", filename);
        let path = Path::new(&complete_path);
        let mut file = File::open(&path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut word : Word = serde_json::from_str(contents.as_str())?;
        word.meanings = word.meanings.into_iter().map(|mut m| {
            m.definition = markdown::to_html(m.definition.as_str());
            m.examples = m.examples.into_iter().map(|e| {
                markdown::to_html(e.as_str())
            }).collect();
            m
        }).collect();
        Ok(word)
    }
}
