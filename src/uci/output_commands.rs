use std::env;

pub fn identify_engine() {
    match env::var("ENGINE_NAME") {
        Ok(name) => println!("id name {}", name),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn identify_author() {
    match env::var("AUTHOR_NAME") {
        Ok(name) => println!("id author {}", name),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn uciok() {
    println!("uciok");
}

pub fn ready() {
    println!("readyok");
}