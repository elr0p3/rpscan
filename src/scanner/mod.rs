extern crate num_cpus;
extern crate itertools;
use itertools::Itertools;
use threadpool::ThreadPool;

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
    total_threads: u8,

    range_ports: Vec<RangePorts>,
    indiv_ports: Vec<u16>,

    ports_to_scan: u16,
    port_chunk: u16,

    indiv_jobs: u16,
    range_jobs: u16,
    total_jobs: u16,
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
        
        let mut range_ports: Vec<RangePorts> =
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

        // Calculate number of threads to use for each port segment
        let mut total_jobs: u16 = 0;
        let port_chunk = ports_to_scan as f32 / threads as f32;
        for rng_prt in range_ports.iter_mut() {
            let t = (rng_prt.get_num() as f32 / port_chunk).ceil() as u8;
            rng_prt.set_threads_to_use(t);
            total_jobs += t as u16;
        }
        let indiv_jobs = (indiv_ports.len() as f32 / port_chunk).ceil() as u16;
        let range_jobs: u16 = total_jobs;
        total_jobs += indiv_jobs;


        Ok(Scanner{
            address,
            total_threads: threads,
            range_ports,
            indiv_ports,
            ports_to_scan,
            port_chunk: port_chunk.ceil() as u16,
            indiv_jobs,
            range_jobs,
            total_jobs,
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


    /// Scan method
    pub fn scan (&self) {
        let (tx, rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
        let n_workers = self.total_threads as usize;
        // let n_jobs = self.total_jobs;
        let pool = ThreadPool::new(n_workers);
        // let mut position: u16 = 0;
        let self_ref = Arc::new(self.clone());

        // Individual ports section
        for i in 0..self.indiv_jobs {
            let tx = tx.clone();
            let indiv_slice = self.divide_indiv_ports(i);
            let self_ref = Arc::clone(&self_ref);
            pool.execute(move|| {
                self_ref.scan_indiv(&indiv_slice, tx);
                // println!("indv {} - {}", i, is);
            });
        }

        // Range ports section
        for i in 0..self.range_jobs {
            let tx = tx.clone();
            pool.execute(move|| {
                tx.send(i).unwrap();
                // println!("jobs {}", i);
            });
        }


        drop(tx);
        let mut open_ports: HashSet<u16> = HashSet::new();
        for port in rx {
            open_ports.insert(port);
        }

        println!("Open Ports for {}:\n{:?}", self.address, open_ports);
    }

    fn divide_indiv_ports (&self, iteration: u16) -> Vec<u16> {
        let pc: u16 = self.port_chunk;
        let actual = (pc * iteration) as usize;

         if iteration == self.indiv_jobs - 1 {  // Last iteration
            let tmp = Vec::from(&self.indiv_ports[actual..]);
            println!("LAST   --- {:?}", tmp);
            tmp

        } else {                                // Middle iteration
            let next = (pc * (iteration + 1)) as usize;
            let tmp = Vec::from(&self.indiv_ports[actual..next]);
            println!("MIDDLE --- {:?}", tmp);
            tmp
        }
    }

    fn scan_indiv (&self, port_list: &[u16], tx: Sender<u16>) {
        for port in port_list {
            match TcpStream::connect_timeout(
                &SocketAddr::new(IpAddr::V4(self.address), *port), Duration::from_millis(10)
                ) {
                    Ok(_) => {
                        println!("- {}", *port);
                        tx.send(*port).unwrap();
                    },
                    Err(_) => {},
            }
        }
    }

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
