#!/usr/bin/env python3

import sys

def main():
    mapped = load("description-database.txt")
    
    # Forever, until we break.
    while True:
        print("Enter geology descriptions: ")
        print("Comma separated, type 'q' to quit.")
        names = input(">> ").strip()
        
        # Break here, if we want to quit.
        if not names or names.lower().startswith('q'):
            break
        
        names = [name.strip() for name in names.split(",")]
        
        print("")
        
        for name in names:
            print(name + ":")
            
            if name in mapped:
                for abbr in mapped[name]:
                    print("->", abbr)
            else:
                print(":: no results.")
        
        print("")


def load(path):
    # Key/value map: description to list of abbreviations.
    mapped = {}
    
    # Open file.
    with open(path, "r") as db:
        # Load each mapping.
        for token in db.read().split(","):
            # Split and clean up strings.
            parts = [part.strip() for part in token.split("=")]
            
            # Store.
            name, *descriptions = parts
            
            for desc in descriptions:
                mapped[desc] = name
            
    return mapped


if __name__ == "__main__":
    main()
