use std::fs::File;
use std::io::prelude::*;
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
