
use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub type Mapping = HashMap<String, String>;

pub fn replace_list(mapping: &Mapping, phrases: &String) -> String {
    let mut converted: Vec<String> = Vec::new();
    
    for phrase in phrases.split(",") {
        let trimmed = String::from(phrase.trim());
        converted.push(replace(&mapping, &trimmed));
    }
    
    return converted.join(", ");
}


pub fn load(path: &str) -> Mapping {
    let re = Regex::new("[\n\r]").unwrap();
    
    let mut map: Mapping = HashMap::new();
    
    let contents = fs::read_to_string(path).expect("Failed to database file");
    let contents = String::from(re.replace_all(contents.as_ref(), ""));
    
    for token in contents.split(",") {
        let mut name = String::new();
        
        for part in token.split("=") {
            if name.is_empty() {
                name = String::from(part.trim());
            }
            else {
                map.insert(String::from(part.trim()), name.clone());
            }
        }
    }
    
    let mut extra: Mapping = HashMap::new();
    
    for (phrase, abbr) in &map {
        if phrase.contains(" ") {
            extra.insert(replace(&map, phrase), abbr.to_string());
        }
    }
    
    return map;
}


pub fn replace(map: &Mapping, phrase: &String) -> String {
    let mut updated = phrase.clone();
    
    for (fragment, abbr) in map {
        if updated.contains(fragment) {
            updated = updated.replace(fragment, abbr);
        }
    }
    // println!("{}", updated);
    return updated.replace(" to ", "-");
}
