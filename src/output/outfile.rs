// use std::error::Error;
// use std::collections::HashMap;


const NORMAL: u8 = b'N';


use super::Output;

pub struct Outfile<'a> {
    mode: char,
    output: &'a Output<'a>,
    // pinfo: HashMap<String, PortInformation>,
}

// struct PortInformation {
    // name: String,
    // tcp: String,
    // udp: String,
    // description: String,
    // sctp: bool,
// }


impl<'a> Outfile<'a> {

    pub fn new (output: &'a Output, mode: char) -> Self {
        Self {
            mode,
            output,
        }
    }

    pub fn is_valid_mode (mode: &[u8]) -> Option<char> {
        match mode[0] {
            NORMAL => Some(NORMAL as char),
            _ => None,
        }
    }
}
