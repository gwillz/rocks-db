extern crate regex;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

type Mapping = HashMap<String, String>;

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
    
    pub fn load(&mut self, path: &str) {
        let re = Regex::new("[\n\r]").unwrap();
        
        let contents = String::from(re.replace_all(
            fs::read_to_string(path)
                .expect("Failed to database file")
                .as_ref(),
            ""));
        
        for token in contents.split(",") {
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
        let mut updated = phrase.clone();
        
        for phrase in &self.phrases {
            if updated.contains(phrase) {
                let abbr = self.map.get(phrase).unwrap();
                updated = updated.replace(phrase, abbr);
            }
        }
        
        return updated.replace(" to ", "-");
    }
    
    pub fn replace_list<'a>(&self, phrases: impl Iterator<Item=&'a str>) -> String {
        let mut converted: Vec<String> = Vec::new();
        
        for phrase in phrases {
            let trimmed = String::from(phrase.trim());
            converted.push(self.replace(&trimmed));
        }
        
        return converted.join(", ");
    }
    
}

impl fmt::Display for RockDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (fragment, abbr) in &self.map {
            write!(f, "{} = {}\n", fragment, abbr)?;
        }
        write!(f, "")
    }
}