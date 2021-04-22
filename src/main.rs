use clap::{App, load_yaml};
#[allow(unused_imports)]
use serde_json;
use num_cpus;


mod scanner;
use scanner::Scanner;
// use scanner::error::ScannerError;
mod output;
use output::{
    Output,
    outfile::Outfile,
};

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // Scanner arguments
    let address = app.value_of("address").unwrap();
    let threads = app.value_of("threads").unwrap().parse::<u8>().unwrap_or(num_cpus::get_physical() as u8);
    let ports   = app.values_of("ports").unwrap().collect::<Vec<&str>>();
    let timeout = app.value_of("timeout").unwrap().parse::<u64>().unwrap_or(10);
    let verbose = app.occurrences_of("verbose") as u8;

    // Output arguments
    let outfile_type = app.value_of("outfile").unwrap_or("\0");

    // Check that the outfile mode is correct
    let out_type = Outfile::is_valid_mode(outfile_type.as_bytes());

    // Start everything related to the scanner
    let scanner = Scanner::new(
        address, threads, &ports, timeout, verbose
    ).unwrap();
    println!("{:#?}", scanner);
    let (address, oports) = scanner.scan();

    // Once scan is done, display result information to the user
    let output = Output::new(&address, &oports);
    if let Some(out) = out_type {
        let outfile = Outfile::new(&output, out);
    }
}
