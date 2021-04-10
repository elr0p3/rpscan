use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
        TcpStream
    },
    str::FromStr,
    collections::HashSet,
    thread,
    sync::{
        Arc,
        mpsc::{
            self,
            Sender,
            Receiver
        },
    },
    time::Duration,
};


const LOCALHOST: &'static str = "localhost";

#[derive(Debug, Clone)]
pub struct Scanner {
    address: Ipv4Addr,
    threads: u8,
    low_port: u16,
    high_port: u16,
    port_list: Vec<u16>,

    open_ports: HashSet<u16>,
}



impl Scanner {

    /// New instance  
    pub fn new (address: &str, mut threads: u8, ports: &[&str]) -> Result<Self, &'static str> {
        let address = if address == LOCALHOST {
            Ipv4Addr::LOCALHOST
        } else {
            match Ipv4Addr::from_str(address) {
                Ok(a) => a,
                Err(_) => return Err("invalid IP address syntax"),
            }
        };
        let mut low_port: u16 = 0;
        let mut high_port: u16 = 0;
        let mut port_hash: HashSet<u16> = HashSet::new();
        let ports_len = ports.len();

        if ports_len == 1 && ports[0].contains("-") {
            if let Err(err) = Self::range_ports(&mut threads, &mut low_port, &mut high_port, ports) {
                return Err(err);
            };
            threads = if ((high_port - low_port) as u8) < threads { ports_len as u8 } else { threads };

            Ok(Self{
                address,
                threads,
                low_port,
                high_port,
                port_list: Vec::new(),
                open_ports: HashSet::new(),
            })
        } else if ports_len > 1 || !ports[0].contains("-") {
            for port in ports {
                let port: u16 = port.parse().unwrap();
                port_hash.insert(port);
            }
            threads = if (ports_len as u8) < threads { ports_len as u8 } else { threads };

            Ok(Self{
                address,
                threads,
                low_port,
                high_port,
                port_list: port_hash.into_iter().collect(),
                open_ports: HashSet::new(),
            })
        } else {
            Err("invalid PORT arguments")
        }
    }

    fn range_ports (threads: &mut u8, lp: &mut u16, hp: &mut u16, ports: &[&str]) -> Result<(), &'static str> {
        let ports_splited = ports[0].split("-").collect::<Vec<_>>();
        if ports_splited.len() != 2 {
            return Err("bad use of range ports");
        } else {
            *lp = ports_splited[0].parse().unwrap();
            *hp = ports_splited[1].parse().unwrap();
            if lp > hp {
                *hp ^= *lp;
                *lp ^= *hp;
                *hp ^= *lp;
            }
        }
        let result = *hp - *lp;
        *threads = if (result as u8) < *threads { result as u8 } else { *threads };
        Ok(())
    }


    /// Scan method
    pub fn scan (&mut self) {
        let (tx, rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
        let sc = self.clone();
        let sc = Arc::new(sc);

        if self.port_list.len() == 0 {
            Self::scan_range(sc, tx);
        } else {
            Self::scan_list(sc, tx);
        }

        // drop(tx);
        for port in rx {
            self.open_ports.insert(port);
        }

        println!("Open Ports for {}:\n{:?}", self.address, self.open_ports);
    }

    fn scan_range (sc: Arc<Self>, tx: Sender<u16>) {
        for i in 0..sc.threads {
            let tx = tx.clone();
            let sc_clone = Arc::clone(&sc);
            thread::spawn(move|| {
                Scanner::connect_host_range(
                    tx, sc_clone.address, sc_clone.low_port, sc_clone.high_port, sc_clone.threads, i
                );
            });
        }
    }

    fn scan_list (sc: Arc<Self>, tx: Sender<u16>) {
        // let divide = ((sc.port_list.len() as f32) / sc.threads as f32).floor() as usize;
        // let divide = ((sc.port_list.len() as f32) / sc.threads as f32).ceil() as usize;
        let mut slices: Vec<Vec<u16>> = vec![Vec::new(); sc.threads as usize];

        for i in 0..sc.port_list.len() {
            let port = sc.port_list[i];
            slices[i % sc.threads as usize].push(port);
        }

        for slice in slices {
            let tx = tx.clone();
            let sc_clone = Arc::clone(&sc);
            thread::spawn(move|| {
                Scanner::connect_host_list(
                    tx, sc_clone.address, &slice
                );
            });
        }
    }

    fn connect_host_range (tx: Sender<u16>, addr: Ipv4Addr, start: u16, end: u16, threads: u8, position: u8) {
        let mut port = start + position as u16;

        loop {
            match TcpStream::connect_timeout(
                &SocketAddr::new(IpAddr::V4(addr), port), Duration::from_millis(10)
                ) {
                    Ok(_) => {
                        println!("- {}", port);
                        tx.send(port).unwrap();
                    },
                    Err(_) => {}
            };

            if end - port <= threads as u16 {
                break;
            }
            port += threads as u16;
        }
    }

    fn connect_host_list (tx: Sender<u16>, addr: Ipv4Addr, port_slice: &[u16]) {
        for port in port_slice {
            match TcpStream::connect_timeout(
                &SocketAddr::new(IpAddr::V4(addr), *port), Duration::from_millis(10)
                ) {
                    Ok(_) => {
                        println!("- {}", *port);
                        tx.send(*port).unwrap();
                    },
                    Err(_) => {},
            }
        }
    }

}
