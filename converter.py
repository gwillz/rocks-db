#!/usr/bin/env python3

import sys

def main():
    mapped = load("description-database.txt")
    
    print("Enter geology descriptions: ")
    print("Comma separated, type 'q' to quit.")
    
    # Forever, until we break.
    while True:
        phrases = input(">> ").strip()
        
        # Break here, if we want to quit.
        if not phrases or phrases.lower().startswith('q'):
            break
        
        # Split and clean.
        phrases = [n.strip() for n in phrases.split(",")]
        shorter = []
        
        print("")
        print("The acronyms are:")
        
        for phrase in phrases:
            shorter.append(replace(mapped, phrase))
        
        print(", ".join(shorter))
        print("")


def replace(mapped, phrase):
    for fragment, abbr in mapped.items():
        if fragment not in phrase: continue
        phrase = phrase.replace(fragment, abbr)
    
    return phrase.replace(" to ", "-")


def load(path):
    # Key/value map: description to list of abbreviations.
    mapped = {}
    
    # Open file.
    with open(path, "r") as db:
        # Load the file and clean out line breaks.
        contents = db.read().replace(r"[\r\n]", "")
        
        # Load each mapping.
        for token in contents.split(","):
            # Split and clean up strings.
            parts = [part.strip() for part in token.split("=")]
            
            # The first part is the abbreviation.
            # All following parts are alternate descriptions.
            abbreviation, *fragments = parts
            
            # Store.
            for fragment in fragments:
                mapped[fragment] = abbreviation
    
    # Fix up duplicates.
    extra = {}
    for phrase, abbr in mapped.items():
        if phrase.count == 0: continue
        
        phrase = replace(mapped, phrase)
        extra[phrase] = abbr
    
    mapped.update(extra)
    
    return mapped


if __name__ == "__main__":
    main()
