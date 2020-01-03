extern crate regex;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{Error, ErrorKind};

use std::ptr;
use std::ffi::{CString, CStr};
use libc::c_char;

type Mapping = HashMap<String, String>;

lazy_static! {
    static ref CLEAN: Regex = Regex::new("[\n\r]").unwrap();
}

pub struct RockDB {
    // A map of fragments to abbreviations.
    map: Mapping,
    // An size ordered list of fragments (biggest to smallest).
    fragments: Vec<String>,
}

impl RockDB {
    // Empty DB constructor.
    pub fn new() -> RockDB {
        RockDB {
            fragments: Vec::new(),
            map: HashMap::new(),
        }
    }
    
    // Load and parse a database from a text file.
    // The format is "abbr1=fragment1, abbr2=fragment2".
    pub fn load(&mut self, path: &str) -> Result<(), Error> {
        match fs::read_to_string(path) {
            Ok(contents) => Result::Ok(self.process(contents)),
            Err(_) => Result::Err(Error::new(
                ErrorKind::NotFound, 
                format!("Failed to load database: {}.", path)
            )),
        }
    }
    
    // Parse the database file.
    fn process(&mut self, contents: String) {
        // Clean and split into fragments.
        for fragment in clean_input(contents).split(",") {
            let mut abbr = String::new();
            
            // Split into parts:
            for part in fragment.split("=") {
                // The part first is the abbreviation.
                if abbr.is_empty() {
                    abbr = String::from(part.trim());
                }
                // All following parts are fragments that will convert into
                // the abbreviation.
                else {
                    let fragment = part.trim().to_string();
                    self.fragments.push(fragment.clone());
                    self.map.insert(fragment, abbr.clone());
                }
            }
        }
        
        // Sort longest to shortest.
        // Being an iterative process, the larger fragments are at risk of
        // being polluted by smaller fragments. With this, the larger get
        // priority.
        self.fragments.sort_by(|a, b| b.len().cmp(&a.len()));
        
        // Breaking down larger fragments.
        // @todo Not sure if this is necessary atm.
        
        // let mut extra: Mapping = HashMap::new();
        // for phrase in &self.phrases {
        //     if phrase.contains(" ") {
        //         let abbr = self.map.get(phrase).unwrap();
        //         extra.insert(self.replace(phrase), abbr.to_string());
        //     }
        // }
        // self.map.extend(extra);
    }
    
    // Convert all fragments into abbreviations.
    pub fn convert(&self, phrase: &String) -> String {
        let mut updated = phrase.clone();
        
        // Iteratively replace. Note, 'fragments' is size-ordered so larger
        // fragments are preserved, but also smaller fragments can post-modify
        // changes made by the larger fragments.
        for fragment in self.fragments.iter() {
            if updated.contains(fragment) {
                let abbr = self.map.get(fragment).unwrap();
                updated = updated.replace(fragment, abbr);
            }
        }
        
        // Special case for 'to'.
        updated = updated.replace(" to ", "-");
        
        return updated;
    }
}

// Remove newlines, trim.
pub fn clean_input(text: String) -> String {
    CLEAN.replace_all(text.as_ref(), "").trim().to_string()
}

impl fmt::Display for RockDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (fragment, abbr) in &self.map {
            write!(f, "{} = {}\n", fragment, abbr)?;
        }
        write!(f, "")
    }
}

// C interfaces

#[no_mangle]
pub unsafe extern "C" fn rocks_load(c_filename: *const c_char) -> *mut RockDB {
    let mut db = RockDB::new();
    
    match CStr::from_ptr(c_filename).to_str() {
        Ok(filename) => {
            match db.load(&String::from(filename)) {
                Ok(_) => Box::into_raw(Box::new(db)),
                Err(_) => ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rocks_convert(db: *const RockDB, c_phrase: *const c_char) -> *const c_char {
    
    match CStr::from_ptr(c_phrase).to_str() {
        Ok(phrase) => {
            match CString::new((*db).convert(&String::from(phrase)).as_str()) {
                Ok(s) => s.into_raw(),
                Err(_) => ptr::null(),
            }
        }
        Err(_) => ptr::null(),
    }
}
