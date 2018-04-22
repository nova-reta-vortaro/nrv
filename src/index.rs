use std;
use std::process::Command;
use std::env;
use rand;
use rand::Rng;

pub struct Index {
    words: Vec<String>
}

impl Index {
    pub fn new () -> Index {
        let paths = std::fs::read_dir("articles").unwrap();

        Index {
            words: paths.filter_map(|path| {
                let file = String::from(format!("{}", path.unwrap().path().display()));
                if file.ends_with(".json") {
                    Some(file.replace("articles/", "").replace(".json", ""))
                } else {
                    None
                }
            }).collect()
        }
    }

    pub fn filter (&self, search: &str) -> Vec<String> {
        let mut res = Vec::new();

        for word in self.words.clone() {
            if word.contains(search) {
                res.push(word.clone())
            }
        }

        res
    }

    pub fn random (&self) -> String {
        let index = rand::thread_rng().gen_range(0, self.words.len());
        self.words[index].clone()
    }

    pub fn import (&mut self, word: String) {
        Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", env::var("IMPORT_CMD").unwrap(), word))
            .spawn()
            .expect("failed to execute process");
        self.words.push(word);
    }
}
