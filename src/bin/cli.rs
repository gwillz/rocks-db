
use std::io::{self, Write};

#[path="../lib.rs"]
mod lib;

fn main() {
    let mapping = lib::load("description-database.txt");
    
    // for (fragment, abbr) in &mapping {
    //    println!("{} = {}", fragment, abbr);
    // }
    // println!("{}", mapping.len());
    
    println!("Enter geology descriptions: ");
    println!("Comma separated, type 'q' to quit.");
    
    loop {
        // Read input.
        let phrases = input(">> ").to_lowercase();
        if phrases.starts_with("q") { break }
        
        println!("");
        println!("The acronyms are:");
        println!("{}", lib::replace_list(&mapping, &phrases));
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