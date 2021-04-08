use clap::{App, load_yaml};
#[allow(unused_imports)]
use serde_json;

mod scanner;
use scanner::Scanner;

fn main() {
    let yaml = load_yaml!("../files/input_params.yml");
    let app = App::from_yaml(yaml).get_matches();
    println!("Address: {}", app.value_of("address").unwrap());
    println!("Threads: {}", app.value_of("threads").unwrap());
    println!("Ports: {}", app.value_of("ports").unwrap());
    println!("Ports: {:?}", app.values_of("ports").unwrap().collect::<Vec<&str>>());

    // println!("Nuevo: {:?}", app.values_of("nuevo").unwrap().collect::<Vec<&str>>());

    // println!("addr: {:?}", app.subcommand_matches("address").unwrap());


    let mut scan = Scanner::new(
        app.value_of("address").unwrap(),
        app.value_of("threads").unwrap().parse::<u8>().unwrap(),
        &app.values_of("ports").unwrap().collect::<Vec<&str>>()
    ).unwrap();
    println!("{:#?}", scan);

    scan.scan();
}
