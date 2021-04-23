use clap::{App, load_yaml};
#[allow(unused_imports)]
use serde_json;
use num_cpus;

use std::{
    process,
    collections::HashMap,
};

mod scanner;
use scanner::Scanner;
// use scanner::error::ScannerError;
mod output;
use output::{
    Output,
    outfile::{
        self,
        Outfile,
    }
};

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml).get_matches();
    // assert!(app.is_present("outfile_grepable"));
    // assert!(app.is_present("outfile_normal"));

    // Scanner arguments
    let address = app.value_of("address").unwrap();
    let threads = app.value_of("threads").unwrap().parse::<u8>().unwrap_or(num_cpus::get_physical() as u8);
    let ports   = app.values_of("ports").unwrap().collect::<Vec<&str>>();
    let timeout = app.value_of("timeout").unwrap().parse::<u64>().unwrap_or(10);
    let verbose = app.occurrences_of("verbose") as u8;

    // Output arguments, and validate the introduced file is correct
    let mut outf_content = HashMap::new();
    for (i, of) in outfile::OUTF_NAMES.iter().enumerate() {
        if let Some(value) = app.value_of(of) {
            if Outfile::is_valid_file(value) {
                outf_content.insert(outfile::OUTF_SHORT[i], value);
            } else {
                eprintln!("ERROR! The file '{}' already exists", value);
                process::exit(1);
            }
        }
    }
    println!("{:?}", outf_content);

    // Start everything related to the scanner
    let scanner = Scanner::new(
        address, threads, &ports, timeout, verbose
    ).unwrap();
    println!("{:#?}", scanner);
    let scanned = scanner.scan();

    // Once scan is done, display result information to the user
    let output = Output::new(&scanned).unwrap();
    println!("{}", output.string_port_result());
    // for (mode, path) in outf_content.iter() {
        // let outf = Outfile::new(&output, *mode, path);
    // }
}
