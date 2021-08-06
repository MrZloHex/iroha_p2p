use super::fw::{connect_to_list_of_peers, get_peers, is_peers};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Peer {
    // Frequency of speaking
    period: u64,
    // IP address of Peer
    address: String,
    // Filename with Peers
    filename: String,
}

impl Peer {
    fn new(per: &str, port: &str, filename: String) -> Peer {
        // IP address of Peer
        let address = format!("127.0.0.1:{}", port);
        println!("My address is '{}'", address);

        // Connecting to file with Peers
        connect_to_list_of_peers(filename.clone(), address.clone());
        Peer {
            period: per.parse::<u64>().unwrap(),
            address,
            filename,
        }
    }

    pub fn start(per: &str, port: &str, filename: Option<&str>) {
        // Connecting to files with all peers
        let filename = if let Some(fname) = filename {
            fname
        } else {
            "Peer.txt"
        };
        // Creating instance of Peer
        let peer = Peer::new(per, port, filename.to_string());
        // Creating TcpListener
        let listen = TcpListener::bind(peer.address.clone()).unwrap();

        // Making new thread for parallel Speaking and Listening
        thread::spawn(move || {
            loop {
                // Speak and wait
                peer.speak();
                thread::sleep(Duration::from_secs(peer.period));
            }
        });

        // Listen for income messages
        for stream in listen.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        // Hadling incoming message
                        handle_income(stream);
                    });
                }
                Err(e) => println!("ERORR TO LISTEN {}", e),
            }
        }
    }

    fn speak(&self) {
        if is_peers(&self.filename) {
            for peer in get_peers(&self.filename) {
                // Speaking for each peer except myself
                if !peer.eq(&self.address) {
                    // Connecting to Peer
                    match TcpStream::connect(peer.clone()) {
                        Ok(mut stream) => {
                            let msg = format!("{}{}", self.address.chars().count(), self.address);
                            // Send IP address of sender and length of its IP
                            stream.write(msg.as_bytes()).unwrap();
                            println!("SPEAK TO {}", peer);
                        }
                        Err(e) => {
                            println!("ERORR TO SPEAK {}", e);
                        }
                    }
                }
            }
        }
    }
}

fn handle_income(mut stream: TcpStream) {
    let mut buffer = [0_u8; 25];
    // Reading income message
    stream.read(&mut buffer).unwrap();
    let size = format!("{}", String::from_utf8_lossy(&buffer[0..2]))
        .parse::<usize>()
        .unwrap();
    let address = format!("{}", String::from_utf8_lossy(&buffer[2..size + 2]));
    println!("LISTEN FROM {}", address);
}
