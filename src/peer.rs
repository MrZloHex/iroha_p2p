use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::Write;
use super::fw::{get_peers, is_peers, create_list_of_peers};


#[derive(Clone)]
pub struct Peer {
    period: u64,
    address: String,
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
            address: address,
            my_peers: my_f
        }

    }

    pub fn start(per: &str, port: &str, connect: Option<&str>) {
        let peer = Peer::new(per, port, connect);
        let listen = TcpListener::bind(peer.address.clone()).unwrap();
        thread::spawn(move|| {
            loop {
                peer.speak();
                thread::sleep(Duration::from_secs(peer.period));
           };
        });

        // Listen part
        //loop {
        for stream in listen.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    //thread::spawn(move|| {
                    //    // connection succeeded
                    //    handle_client(stream)
                    //});
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        //}
        // close the socket server
    }


    fn speak(&self) {
        if is_peers(self.my_peers.clone()) {
            for peer in get_peers(self.my_peers.clone()) {
                match TcpStream::connect(peer) {
                    Ok(mut stream) => {
                        let msg = b"Hello!";
                        stream.write(msg).unwrap();
                    },
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
        }
    }
}
