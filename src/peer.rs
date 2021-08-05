use std::net::{TcpListener, TcpStream};

pub struct Peer {
    period: u16,
    me: TcpListener,
    peers: Option<Vec<TcpStream>>
}

impl Peer{
    pub fn new(per: &str, port: &str, init_peer: Option<&str>) -> Peer {
        let address = format!("127.0.0.1:{}", port);
        println!("{}", address);

        let mut peer = if let Some(an_peer) = init_peer {
            let stream = match TcpStream::connect(an_peer) {
                Ok(mut stream) => stream,
                Err(e) => panic!("CAN'T CONNECT {}", e)
            };


            Peer {
                period: per.parse::<u16>().unwrap(),
                me: TcpListener::bind(address).unwrap(),
                peers: Some(vec![stream])
            }
        } else {
            Peer { 
                period: per.parse::<u16>().unwrap(),
                me: TcpListener::bind(address).unwrap(),
                peers: None
            }
        };
        peer

    }

    pub fn start_communication(&mut self) {

    }
}
