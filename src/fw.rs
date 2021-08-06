use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
    BufRead
};
use std::path::Path;


pub fn create_list_of_peers(filename: String, connect: Option<&str>) {
    let path = Path::new(&filename);
    let mut file = match File::create(&path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f
    };
    
    if let Some(peer) = connect {
        file.write_all(peer.as_bytes()).unwrap();
    }
}


pub fn is_peers(filename: String) -> bool {
    let path = Path::new(&filename);
    let mut file = match File::open(&path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(e) => panic!("{}", e),
        _ => ()
    };
    if content.is_empty() {
        false
    } else {
        true
    }
}


pub fn get_peers(filename: String) -> Vec<String> {
    let path = Path::new(&filename);
    let file = match File::open(&path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f
    };

    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }
    lines
}
