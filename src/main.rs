extern crate clap;
use clap::{App, load_yaml};

mod fw;
mod peer;
use peer::Peer; 

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let period = if let Some(period) = matches.value_of("period") {
        period
    } else {"5"};
    let port = if let Some(port) = matches.value_of("port") {
        port
    } else {"8080"};
    let connect: Option<&str> = matches.value_of("connect");

    let peer = Peer::new(period, port, connect);
    peer.start_communication();
}

