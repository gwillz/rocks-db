extern crate regex;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{Error, ErrorKind};

type Mapping = HashMap<String, String>;

lazy_static! {
    static ref RE: Regex = Regex::new("[\n\r]").unwrap();
}

pub struct RockDB {
    phrases: Vec<String>,
    map: Mapping,
}

impl RockDB {
    pub fn new() -> RockDB {
        RockDB {
            phrases: Vec::new(),
            map: HashMap::new(),
        }
    }
    
    pub fn load(&mut self, path: &str) -> Result<(), Error> {
        match fs::read_to_string(path) {
            Ok(contents) => Result::Ok(self.process(&contents)),
            Err(_) => Result::Err(Error::new(ErrorKind::NotFound, format!("Failed to load database: {}.", path))),
        }
    }
    
    fn process(&mut self, contents: &String) {
        for token in clean_input(contents).split(",") {
            let mut abbr = String::new();
            
            for part in token.split("=") {
                if abbr.is_empty() {
                    abbr = String::from(part.trim());
                }
                else {
                    let phrase = String::from(part.trim());
                    self.phrases.push(phrase.clone());
                    self.map.insert(phrase, abbr.clone());
                }
            }
        }
        
        // Sort longest to shortest.
        self.phrases.sort_by(|a, b| b.len().cmp(&a.len()));
        
        let mut extra: Mapping = HashMap::new();
        
        for (phrase, abbr) in &self.map {
            if phrase.contains(" ") {
                extra.insert(self.replace(phrase), abbr.to_string());
            }
        }
        
        self.map.extend(extra);
    }
    
    pub fn replace(&self, phrase: &String) -> String {
        let mut updated = clean_input(phrase);
        
        for phrase in &self.phrases {
            if updated.contains(phrase) {
                let abbr = self.map.get(phrase).unwrap();
                updated = updated.replace(phrase, abbr);
            }
        }
        
        return updated.replace(" to ", "-");
    }
}

pub fn clean_input(text: &String) -> String {
    RE.replace_all(text.as_ref(), "").trim().to_string()
}

impl fmt::Display for RockDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (fragment, abbr) in &self.map {
            write!(f, "{} = {}\n", fragment, abbr)?;
        }
        write!(f, "")
    }
}