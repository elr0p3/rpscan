// Extern crates
use serde::Deserialize;

// Standard library imports
use std::{
    fs,
    collections::HashMap,
    error::Error,
};

// Crate modules
pub mod outfile;
// pub mod error;
use super::scanner::Scanned;


const PORT_INFO_FILE: &'static str = "./etc/pinfo.test.json";


#[derive(Debug)]
pub struct Output<'a> {
    scanned: &'a Scanned,
    pinfo: HashMap<u16, PortInformation>,
}

#[derive(Deserialize, Debug)]
struct PortInformation {
    name: String,
    tcp: String,
    udp: String,
    description: String,
    sctp: bool,
}



impl<'a> Output<'a> {

    pub fn new (scanned: &'a Scanned) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(PORT_INFO_FILE)?;
        let pinfo = serde_json::from_str(&file_content)?;
        Ok(Self {
            scanned,
            pinfo,
        })
    }

    pub fn string_port_result (&self) -> String {
        let ports = self.scanned.get_ports();
        let closed_ports = u16::MAX - ports.len() as u16;
        let open_ports = ports.len();
        let mut result = format!(
            "\nPort result for '{}':\n Closed: {}\n Open: {}\n\n",
            self.scanned.get_addr(), closed_ports, open_ports
        );

        for p in ports.iter() {
            result += format!("  {}\n", p).as_str();
        }

        result += format!(
            "\nTime spent scanning {} ports: {:?}",
            self.scanned.get_ports_scanned(), self.scanned.get_duration()
        ).as_str();

        result
    }
}
