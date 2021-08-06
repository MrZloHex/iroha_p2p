use std::fs::{OpenOptions, File};
use std::io::{
    prelude::*,
    BufReader,
    BufRead
};
use std::path::Path;


pub fn connect_to_list_of_peers(filename: String, address: String) {
    if is_file(&filename) {
        add_peer(&filename, &address);
    } else {
        create_list_of_peers(&filename);
        add_peer(&filename, &address);
    }
}

fn is_file(filename: &String) -> bool {
    let file: bool = std::path::Path::new(filename).exists();
    file
}

fn create_list_of_peers(filename: &String) {
    let path = Path::new(filename);
    let _file = match File::create(&path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f
    };
}


pub fn is_peers(filename: &String) -> bool {
    let path = Path::new(filename);
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


pub fn get_peers(filename: &String) -> Vec<String> {
    let path = Path::new(filename);
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


fn add_peer(filename: &String, peer: &String) {
    if !is_peer(filename, peer) {
        let path = Path::new(filename);
        let mut file = OpenOptions::new().write(true).append(true).open(&path).unwrap();
        file.write_all(format!("{}\n", peer).as_bytes()).unwrap();
    }
}


fn is_peer(filename: &String, fpeer: &String) -> bool {
    let peers = get_peers(filename);
    let mut output: bool = false;
    for peer in peers {
        if peer.eq(fpeer) {
            output = true;
            break;
        }
    }
    output
}
