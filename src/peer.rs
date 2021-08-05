use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::str::from_utf8;


pub struct Peer {
    period: u64,
    me: TcpListener,
    peers: Option<Vec<TcpStream>>
}

impl Peer{
    pub fn new(per: &str, port: &str, init_peer: Option<&str>) -> Peer {
        let address = format!("127.0.0.1:{}", port);
        println!("{}", address);

        let peer = if let Some(an_peer) = init_peer {
            let stream = match TcpStream::connect(an_peer) {
                Ok(stream) => stream,
                Err(e) => panic!("CAN'T CONNECT {}", e)
            };


            Peer {
                period: per.parse::<u64>().unwrap(),
                me: TcpListener::bind(address).unwrap(),
                peers: Some(vec![stream])
            }
        } else {
            Peer { 
                period: per.parse::<u64>().unwrap(),
                me: TcpListener::bind(address).unwrap(),
                peers: None
            }
        };
        peer

    }

    pub fn start_communication(&mut self) {
        thread::spawn(|| {
            self.speak();
            thread::sleep(Duration::from_secs(self.period));
        });


    }


    fn speak(&mut self) {
        if let Some(peers) = self.peers {
            for peer in peers {
                let msg = b"Hello!";

                peer.write(msg).unwrap();
                println!("Sent Hello, awaiting reply...");

                let mut data = [0 as u8; 6]; // using 6 byte buffer
                match peer.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
    }
}
