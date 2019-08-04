
use std::io::{self, Write};

#[path="../lib.rs"]
mod lib;
use lib::RockDB;

fn main() {
    let mut db = RockDB::new();
    db.load("description-database.txt");
    // println!("{}", db);
    
    println!("Enter geology descriptions: ");
    println!("Comma separated, type 'q' to quit.");
    
    loop {
        // Read input.
        let phrases = input(">> ").to_lowercase();
        if phrases.starts_with("q") { break }
        
        println!("");
        println!("The acronyms are:");
        println!("{}", db.replace_list(phrases.split(",")));
        println!("");
    }
    
    println!("Quitting.");
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
