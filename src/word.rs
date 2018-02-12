use std;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    pub meanings: Vec<Meaning>
}

impl Word {
    pub fn from_file (filename: &str) -> std::io::Result<Word> {
        let complete_path = format!("articles/{}.json", filename);
        let path = Path::new(&complete_path);
        let mut file = File::open(&path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let word : Word = serde_json::from_str(contents.as_str())?;
        Ok(word)
    }
}
