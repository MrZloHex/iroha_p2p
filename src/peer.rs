use std::net::TcpListener;

pub struct Peer {
    period: u16,
    me: TcpListener
}

impl Peer{
    pub fn new(per: &str, port: &str) -> Peer {
        let address = format!("127.0.0.1:{}", port);
        println!("{}", address);
        Peer {
            period: per.parse::<u16>().unwrap(),
            me: TcpListener::bind(address).unwrap()
        }
    }

    pub fn start_communication(&mut self) {

    }
}
