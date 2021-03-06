extern crate regex;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{Error, ErrorKind};

use std::ptr;
use std::ffi::{CString, CStr};
use libc::{c_char, size_t};

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
    
    pub fn from<S: AsRef<str>>(contents: S) -> RockDB {
        let mut db = RockDB::new();
        db.process(contents);
        return db
    }
    
    // Load and parse a database from a text file.
    // The format is "abbr1=fragment1, abbr2=fragment2".
    pub fn load<S: AsRef<str>>(&mut self, path: S) -> Result<(), Error> {
        match fs::read_to_string(path.as_ref()) {
            Ok(contents) => Result::Ok(self.process(contents)),
            Err(_) => Result::Err(Error::new(
                ErrorKind::NotFound, 
                format!("Failed to load database: {}.", path.as_ref())
            )),
        }
    }
    
    // Fragments in alphabetical order.
    pub fn get_fragments(&self) -> Vec<String> {
        let mut copy = self.fragments.clone();
        copy.sort_by(|a, b| a.cmp(&b));
        copy
    }
    
    // Parse the database file.
    fn process<S: AsRef<str>>(&mut self, contents: S) {
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
                    
                    if !self.fragments.contains(&fragment) {
                        self.fragments.push(fragment.clone());
                    }
                    
                    self.map.insert(fragment, abbr.clone());
                }
            }
        }
        
        // Sort longest to shortest.
        // Being an iterative process, the larger fragments are at risk of
        // being polluted by smaller fragments. With this, the larger get
        // priority.
        self.fragments.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        
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
    pub fn convert<S: AsRef<str>>(&self, phrase: S) -> String {
        let mut updated = String::from(phrase.as_ref());
        
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
        // @todo Should this be a regex? Like "\w+to\w+".
        updated = updated.replace(" to ", "-");
        
        return updated;
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

// Remove newlines, trim.
pub fn clean_input<S: AsRef<str>>(text: S) -> String {
    CLEAN.replace_all(text.as_ref(), "").trim().to_string()
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

#[repr(C)]
pub struct Fragments {
    pub items: *const *const c_char,
    pub size: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn rocks_fragments(db: *const RockDB) -> Fragments {
    let mut fragments: Vec<*const c_char> = Vec::new();
    
    for fragment in (*db).get_fragments() {
        if let Ok(c_fragment) = CString::new(fragment.as_str()) {
            fragments.push(c_fragment.into_raw());
        }
    }
    
    let boxed: Box<*const *const c_char> = Box::new(fragments.as_ptr());
    
    Fragments {
        items: *Box::into_raw(boxed),
        size: fragments.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clean_input() {
        let actual = clean_input(" big old \n\r weird\r text \n ");
        let expected = "big old  weird text";
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_simple() {
        let db = RockDB::from("o=one, tw=two, th=three");
        
        let fragments = db.get_fragments();
        
        assert_eq!(fragments.len(), 3);
        assert_eq!(fragments[0], "one");
        assert_eq!(fragments[1], "three");
        assert_eq!(fragments[2], "two");
        
        assert_eq!(db.convert("three"), "th");
        assert_eq!(db.convert("two"), "tw");
        assert_eq!(db.convert("one"), "o");
        assert_eq!(db.convert("four"), "four");
        
        let actual = db.convert("one two half three");
        let expected = "o tw half th";
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_poor_formatting() {
        let db = RockDB::from("o=one=one\n\n,tw = two=2,,th =thre= three  ");
        
        let fragments = db.get_fragments();
        
        assert_eq!(fragments.len(), 5);
        assert_eq!(fragments[0], "2");
        assert_eq!(fragments[1], "one");
        assert_eq!(fragments[2], "thre");
        assert_eq!(fragments[3], "three");
        assert_eq!(fragments[4], "two");
        
        let actual = db.convert("one two 2 three thre");
        let expected = "o tw tw th th";
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_special_case() {
        let db = RockDB::from("o=one, t=two, f=five");
        
        let actual = db.convert("one to five");
        let expected = "o-f";
        
        assert_eq!(actual, expected);
    }
}
