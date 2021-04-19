extern crate num_cpus;
extern crate itertools;
use itertools::Itertools;

pub mod range_port;
use range_port::RangePorts;

// pub mod scerror;

#[allow(unused_imports)]
use std::{
    collections::{
        HashSet,
        HashMap,
    },
    error::Error,
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
        TcpStream
    }, str::FromStr, sync::{
        Arc,
        mpsc::{
            self,
            Sender,
            Receiver
        },
    },
    thread,
    time::Duration,
};


const LOCALHOST: &'static str = "localhost";


#[derive(Debug, Clone)]
pub struct Scanner {
    address: Ipv4Addr,
    threads: u8,

    range_ports: Vec<RangePorts>,
    indiv_ports: Vec<u16>,
    ports_to_scan: u16,
}


impl Scanner {

    /// New instance  
    pub fn new (address: &str, mut threads: u8, ports: &[&str]) -> Result<Self, Box<dyn Error>> {

        // Parse Address
        let address = if address == LOCALHOST {
            Ipv4Addr::LOCALHOST
        } else {
            Ipv4Addr::from_str(address)?
        };

        // Parse Ports
        let mut indiv_hash: HashSet<u16> = ports.iter()
            .filter_map(|p| p.parse().ok())
            .collect();
        
        let range_ports: Vec<RangePorts> =
            Self::parse_range_ports(ports, &mut indiv_hash);

        let indiv_ports: Vec<u16> =
            Self::parse_indiv_ports(&range_ports, &indiv_hash);

        let mut ports_to_scan: u16 = 0;
        for rp in range_ports.iter() {
            ports_to_scan += rp.get_num();
        }
        ports_to_scan += indiv_ports.len() as u16;

        // Check Threads
        let cores = num_cpus::get();
        if (threads as u16) > ports_to_scan && ports_to_scan < cores as u16 {
            threads = ports_to_scan as u8;
        } else if threads > cores as u8{
            threads = cores as u8;
        }

        // if ports_to_scan == 0 {
            // return NoPortsToScan;
        // }

        Ok(Scanner{
            address,
            threads,
            range_ports,
            indiv_ports,
            ports_to_scan,
        })
    }


    // cargo r -- -p2-7,10-20,5-9,10-100 0.0.0.0
    // CHANGE IN THE FUTURE
    fn parse_range_ports (ports: &[&str], indiv_ref: &mut HashSet<u16>) -> Vec<RangePorts> {
        let mut range_p: Vec<RangePorts> = vec![];
        let range_ref = &mut range_p;

        ports.iter()
            .filter(|p| p.parse::<u16>().is_err())
            .filter_map(|range| RangePorts::from_str(range).ok())
            .filter(|rng_prt| {
                let result = rng_prt.same_pair_port();
                if result {
                    indiv_ref.insert(rng_prt.get_low());
                }
                !result
            })
            .sorted_by(|a,b| Ord::cmp(&b.get_high(), &a.get_high()))
            .for_each(|rng_prt| {
                if let Some(position) = range_ref.iter()
                    .position(|rp| rp.can_merge(&rng_prt)) {
                    range_ref[position].change_range_ports(&rng_prt);
                } else {
                    range_ref.push(rng_prt);
                }
            });
        range_p
    }

    fn parse_indiv_ports (range_ref: &[RangePorts], indiv_ref: &HashSet<u16>) -> Vec<u16> {
        indiv_ref
            .into_iter()
            .filter(|&p| !range_ref.iter().any(|r| r.contains(*p)))
            .map(|&p| p)
            .collect()
    }


    // /// Scan method
    // pub fn scan (&mut self) {
        // let (tx, rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
        // // let sc = self.clone();
        // // let sc = Arc::new(sc);


        // // for rp in self.range_ports.iter() {
            // // let ctx = tx.clone();
            // // self.scan_range(rp, ctx);
        // // }


        // // drop(tx);
        // let mut open_ports: HashSet<u16> = HashSet::new();
        // for port in rx {
            // open_ports.insert(port);
        // }

        // println!("Open Ports for {}:\n{:?}", self.address, open_ports);
    // }

    // fn scan_range (&self, rp: &RangePorts, tx: Sender<u16>) {
        // for i in 0..self.threads {
            // let tx = tx.clone();
            // let self_clone = self.clone();
            // let rp_clone = rp.clone();
            // thread::spawn(move|| {
                // Scanner::connect_host_range(
                    // tx, self_clone.address, 
                    // rp_clone.get_low(), rp_clone.get_high(),
                    // self_clone.threads, i
                // );
            // });
        // }
    // }

    // fn scan_list (sc: Arc<Self>, tx: Sender<u16>) {
        // // let divide = ((sc.port_list.len() as f32) / sc.threads as f32).floor() as usize;
        // // let divide = ((sc.port_list.len() as f32) / sc.threads as f32).ceil() as usize;
        // let mut slices: Vec<Vec<u16>> = vec![Vec::new(); sc.threads as usize];

        // for i in 0..sc.port_list.len() {
            // let port = sc.port_list[i];
            // slices[i % sc.threads as usize].push(port);
        // }

        // for slice in slices {
            // let tx = tx.clone();
            // let sc_clone = Arc::clone(&sc);
            // thread::spawn(move|| {
                // Scanner::connect_host_list(
                    // tx, sc_clone.address, &slice
                // );
            // });
        // }
    // }

    // fn connect_host_range (tx: Sender<u16>, addr: Ipv4Addr, start: u16, end: u16, threads: u8, position: u8) {
        // let mut port = start + position as u16;

        // loop {
            // match TcpStream::connect_timeout(
                // &SocketAddr::new(IpAddr::V4(addr), port), Duration::from_millis(10)
                // ) {
                    // Ok(_) => {
                        // println!("- {}", port);
                        // tx.send(port).unwrap();
                    // },
                    // Err(_) => {}
            // };

            // if end - port <= threads as u16 {
                // break;
            // }
            // port += threads as u16;
        // }
    // }

    // fn connect_host_list (tx: Sender<u16>, addr: Ipv4Addr, port_slice: &[u16]) {
        // for port in port_slice {
            // match TcpStream::connect_timeout(
                // &SocketAddr::new(IpAddr::V4(addr), *port), Duration::from_millis(10)
                // ) {
                    // Ok(_) => {
                        // println!("- {}", *port);
                        // tx.send(*port).unwrap();
                    // },
                    // Err(_) => {},
            // }
        // }
    // }

}


#[cfg(test)]
mod scanner_tests {

    use std::collections::HashSet;
    use super::range_port::RangePorts;
    use std::str::FromStr;
    use std::error::Error;

   // cargo t time_spent -- --nocapture

    #[test]
    fn time_spent () -> Result<(), Box<dyn Error>> {

        let mut indiv_hash: HashSet<u16> = HashSet::new();
        for i in 1..=u16::MAX {
            indiv_hash.insert(i);
        }

        let mut range_ports: Vec<RangePorts> = vec![];
        range_ports.push(RangePorts::from_str("1-25")?);
        range_ports.push(RangePorts::from_str("30-70")?);
        range_ports.push(RangePorts::from_str("100-170")?);
        range_ports.push(RangePorts::from_str("300-450")?);
        range_ports.push(RangePorts::from_str("600-1000")?);
        range_ports.push(RangePorts::new(1500, 2777));
        range_ports.push(RangePorts::new(4444, 2));
        range_ports.push(RangePorts::new(5555, 7878));
        range_ports.push(RangePorts::new(10_000, 11_234));
        range_ports.push(RangePorts::new(15_987, 15_999));
        range_ports.push(RangePorts::new(15_987, 15_999));
        range_ports.push(RangePorts::new(16_000, 16_001));
        range_ports.push(RangePorts::new(20_000, 30_000));
        range_ports.push(RangePorts::new(30_030, 36_721));
        range_ports.push(RangePorts::new(42_339, 60_000));
        range_ports.push(RangePorts::new(60_000, u16::MAX));
        range_ports.push(RangePorts::new(1, u16::MAX));

        let start_on = std::time::Instant::now();
        let mut new = Vec::new();
        for port in indiv_hash.iter() {
            for rp in range_ports.iter() {
                if !rp.contains(*port) {
                    new.push(port);
                }
            }
        }
        let duration_on = start_on.elapsed().as_millis();
        println!("Loops O(n^2)\t- {}ms", duration_on);


        let start_fn = std::time::Instant::now();
        let rp_ref = &range_ports;
        let _indvl_ports: Vec<u16> = indiv_hash
            .into_iter()
            .filter(|p| !rp_ref.iter().any(|r| r.contains(*p)))
            .map(|p| p)
            .collect();
        let duration_fn = start_fn.elapsed().as_millis();
        println!("Iterators\t- {}ms", duration_fn);
        
        assert!(duration_fn < duration_on);
        Ok(())
    }

}


/*
 * let mut range_ports: Vec<RangePorts> = Vec::new();
        let mut indvl_ports: HashSet<u16> = HashSet::new();
        let mut to_intro_rng = true;
        let mut to_intro_indv = true;
        for p in ports {
            if p.contains(SEP) {
                let rng_prt = RangePorts::from_str(p)?;
                if rng_prt.same_pair_port() {
                    for rp in range_ports.iter_mut() {
                        if rp.contains(rng_prt.get_low()) {
                            to_intro_indv = false;
                            break;
                        }
                    }
                    if to_intro_indv {
                        indvl_ports.insert(rng_prt.get_low());
                    }
                } else {
                    for rp in range_ports.iter_mut() {
                        if rp.can_merge(&rng_prt) {
                            rp.change_range_ports(&rng_prt);
                            to_intro_rng = false;
                            break;
                        }
                    }
                    if to_intro_rng {
                        range_ports.push(rng_prt);
                    }
                    to_intro_rng = true;
                }
            }
        }
*/
        // let mut range_ports: Vec<RangePorts> = vec![];
        // let mut introduce = true;
        // let tmp_rp: Vec<&&str> = ports.into_iter().filter(|p| p.parse::<u16>().is_err()).collect();
        // for p in tmp_rp {
            // let rng_prt = RangePorts::from_str(*p)?;
            // if rng_prt.same_pair_port() {
                // indiv_hash.insert(rng_prt.get_low());
                // continue;
            // }
            // for rp in range_ports.iter_mut() {
                // if rp.can_merge(&rng_prt) {
                    // rp.change_range_ports(&rng_prt);
                    // introduce = false;
                    // break;
                // }
            // }
            // if introduce {
                // range_ports.push(rng_prt);
            // }
            // introduce = true;
        // }
