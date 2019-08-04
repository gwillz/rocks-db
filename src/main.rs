
use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;
use regex::Regex;

type Mapping = HashMap<String, String>;

fn main() {
    
    let mapping = load("description-database.txt");
    
    for (fragment, abbr) in &mapping {
       println!("{} = {}", fragment, abbr);
    }
    println!("{}", mapping.len());
    
    println!("Enter geology descriptions: ");
    println!("Comma separated, type 'q' to quit.");
    
    loop {
        // Read input.
        let phrases = input(">> ").to_lowercase();
        if phrases.starts_with("q") { break }
        
        println!("");
        println!("The acronyms are:");
        
        let mut converted: Vec<String> = Vec::new();
        
        for phrase in phrases.split(",") {
            let trimmed = String::from(phrase.trim());
            converted.push(replace(&mapping, &trimmed));
        }
        
        println!("{}", converted.join(", "));
        println!("");
    }
    
    println!("Quitting.");
}


fn load(path: &str) -> Mapping {
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


fn replace(map: &Mapping, phrase: &String) -> String {
    let mut updated = phrase.clone();
    
    for (fragment, abbr) in map {
        if updated.contains(fragment) {
            updated = updated.replace(fragment, abbr);
        }
    }
    // println!("{}", updated);
    return updated.replace(" to ", "-");
}


fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .unwrap();
    
    return text;
}