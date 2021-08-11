//extern crate clap;
//use clap::{load_yaml, App};
//
//mod fw;
//mod peer;
//use peer::Peer;

extern crate p2p_from_std_net;
use p2p_from_std_net::Peer;


fn main() {
    // Loading configuration of cli arguments
    //let yaml = load_yaml!("cli.yaml");
    //// Compliance check
    //let matches = App::from_yaml(yaml).get_matches();

    //// Assigment of arguments
    //let period = if let Some(period) = matches.value_of("period") {
    //    period
    //} else {
    //    "5"
    //};
    //let port = if let Some(port) = matches.value_of("port") {
    //    port
    //} else {
    //    "8080"
    //};
    //let filename: Option<&str> = matches.value_of("list");

    //// Starting Peer
    //Peer::start(period, port, filename);


}
