pub mod outfile;

use std::net::IpAddr;

pub struct Output<'a> {
    address: &'a IpAddr,
    open_ports: &'a [u16],
}


impl<'a> Output<'a> {

    pub fn new (address: &'a IpAddr, open_ports: &'a [u16]) -> Self {
        Self {
            address,
            open_ports,
        }
    }
}
