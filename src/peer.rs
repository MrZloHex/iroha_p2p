use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::fs::OpenOptions;

use super::fw::create_list_of_peers;


pub struct Peer {
    period: u64,
    me: TcpListener,
    my_peers: String
}

impl Peer{
    pub fn new(per: &str, port: &str, connect: Option<&str>) -> Peer {
        let address = format!("127.0.0.1:{}", port);
        let my_f = format!("/home/zs/Projects/iroha_p2p/tmp/{}.txt", port);
        println!("{}", address);
        create_list_of_peers(my_f.clone(), connect);
        Peer {
            period: per.parse::<u64>().unwrap(),
            me: TcpListener::bind(address).unwrap(),
            my_peers: my_f
        }

    }

    pub fn start_communication(&'static self) {
        thread::spawn(move|| {
            self.speak();
            thread::sleep(Duration::from_secs(self.period));
        });

        // Listen part
        // for stream in self.me.incoming() {
        //     match stream {
        //         Ok(stream) => {
        //             println!("New connection: {}", stream.peer_addr().unwrap());
        //             thread::spawn(move|| {
        //                 // connection succeeded
        //                 // handle_client(stream)
        //             });
        //         }
        //         Err(e) => {
        //             println!("Error: {}", e);
        //             /* connection failed */
        //         }
        //     }
        // }
    }


    fn speak(&self) {
    }
}
