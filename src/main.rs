use clap::{App, load_yaml};
#[allow(unused_imports)]
use serde_json;

fn main() {
    let yaml = load_yaml!("../files/input_params.yml");
    let app = App::from_yaml(yaml)
        .arg(
            clap::Arg::with_name("nuevo")
                .takes_value(true)
                .last(true))
        .get_matches();
    println!("Address: {}", app.value_of("address").unwrap());
    println!("Threads: {}", app.value_of("threads").unwrap());
    println!("Ports: {}", app.value_of("ports").unwrap());

    // println!("addr: {:?}", app.subcommand_matches("address").unwrap());
    println!("MARIKONG");
}
