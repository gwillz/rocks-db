
use std::io::{self, Write};
use rocks::RockDB;

// Move this to a config? arg parameter?
const DB_PATH: &str = "description-database.txt";

fn main() {
    // Load from file.
    let mut db = RockDB::new();
    db.load(DB_PATH).unwrap();
    // println!("{}", db);
    
    println!("Enter geology descriptions: ");
    println!("Comma separated, type 'q' to quit.");
    
    loop {
        // Read input.
        let phrases = input(">> ").trim().to_lowercase();
        println!("");
        
        // Quit on 'q' or empty.
        if phrases.is_empty() { break }
        if phrases.starts_with("q") { break }
        
        // Process and print.
        println!("The acronyms are:");
        println!("{}", db.convert(&phrases));
        println!("");
    }
    
    println!("Quitting.");
}

// Console input helper.
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .unwrap();
    
    return text;
}
