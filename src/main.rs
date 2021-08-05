extern crate clap;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(period) = matches.value_of("period") {
        println!("{}", period);
    }
}

