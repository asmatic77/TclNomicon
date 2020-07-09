use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

use amethyst::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Enchantment {
    name: String,
    spell: String,
}

fn main() {
    let mut book = Vec::new();

    match std::fs::read_to_string("book.json") {
        Err(_) => println!("No book.json found!"),
        Ok(s) => book = serde_json::from_str(&s).unwrap(),
    };
    

    let name = "Pepe".to_string();
    let spell = "avelina".to_string();
    let ench = Enchantment {name, spell};
    book.push(ench);

    book.push(Enchantment{name:"julio".to_string(), spell: "caesar".to_string()});

    println!("Spells in the book {}:", book.len());
    for s in 0..book.len() {
        println!("{:?}", book[s]);
    }

    let serialized = serde_json::to_string(&book).unwrap();
    println!("The serialization looks like {}", serialized);

    let path = Path::new("book.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
