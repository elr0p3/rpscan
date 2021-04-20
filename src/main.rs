use clap::{App, load_yaml};
#[allow(unused_imports)]
use serde_json;
use num_cpus;

mod scanner;
use scanner::Scanner;
// use scanner::scerror::ScannerError;

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml).get_matches();
    println!("Address: {}", app.value_of("address").unwrap());
    println!("Threads: {}", app.value_of("threads").unwrap());
    println!("Ports: {}", app.value_of("ports").unwrap());
    println!("Ports: {:?}", app.values_of("ports").unwrap().collect::<Vec<&str>>());

    // println!("addr: {:?}", app.subcommand_matches("address").unwrap());


    let address = app.value_of("address").unwrap();
    let threads = app.value_of("threads").unwrap().parse::<u8>().unwrap_or(num_cpus::get_physical() as u8);
    let ports = app.values_of("ports").unwrap().collect::<Vec<&str>>();

    let scanner = Scanner::new(address, threads, &ports).unwrap();
    println!("{:#?}", scanner);

    scanner.scan();
}
