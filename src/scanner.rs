use std::{
    fmt,
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
        Arc, Mutex,
        mpsc::{
            self,
            Sender,
            Receiver
        },
    },
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct Scanner {
    address: Ipv4Addr,
    threads: u8,
    low_port: u16,
    high_port: u16,
    port_list: HashSet<u16>,

    open_ports: HashSet<u16>,
}



impl Scanner {

    /// New instance  
    pub fn new (address: &str, mut threads: u8, ports: &[&str]) -> Result<Self, &'static str> {
        let address = match Ipv4Addr::from_str(address) {
            Ok(a) => a,
            Err(_) => return Err("invalid IP address syntax"),
        };
        let mut low_port: u16 = 0;
        let mut high_port: u16 = 0;
        let mut port_list: HashSet<u16> = HashSet::new();
        let ports_len = ports.len();

        if ports_len == 1 && ports[0].contains("-") {
            let ports_splited = ports[0].split("-").collect::<Vec<_>>();
            if ports_splited.len() != 2 {
                return Err("bad use of range ports");
            } else {
                low_port = ports_splited[0].parse().unwrap();
                high_port = ports_splited[1].parse().unwrap();
                if low_port > high_port {
                    high_port ^= low_port;
                    low_port ^= high_port;
                    high_port ^= low_port;
                }
            }
            let result = high_port - low_port;
            threads = if (result as u8) < threads { result as u8 } else { threads };
        } else {
            for port in ports {
                let port: u16 = port.parse().unwrap();
                port_list.insert(port);
            }
            threads = if (ports_len as u8) < threads { ports_len as u8 } else { threads };
        }

        if port_list.len() == 0 {
            Ok(Self{
                address,
                threads,
                low_port,
                high_port,
                port_list: HashSet::new(),
                open_ports: HashSet::new(),
            })
        } else {
            Ok(Self{
                address,
                threads,
                low_port,
                high_port,
                port_list,
                open_ports: HashSet::new(),
            })
        }
    }


    /// Scan method
    pub fn scan (&mut self) {
        let (tx, rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
        let sc = self.clone();
        // let sc = Arc::new(Mutex::new(sc));
        let sc = Arc::new(sc);
        let mut handles = vec![];

        for i in 0..self.threads {
            let tx = tx.clone();
            let sc_clone = Arc::clone(&sc);

            // if self.port_list.len() == 0 {
                let handle = thread::spawn(move|| {
                    // let sc_lock = sc_clone.lock().unwrap();
                    // Scanner::connect_host_range(
                        // tx, sc_lock.address, sc_lock.low_port, sc_lock.high_port, sc_lock.threads, i
                    // );
                    Scanner::connect_host_range(
                        tx, sc_clone.address, sc_clone.low_port, sc_clone.high_port, sc_clone.threads, i
                    );
                });
                handles.push(handle);
            // } else {
                // thread::spawn(move|| {
                    // Scanner::connect_host_list(tx);
                // });
            // }
        }

        drop(tx);
        for port in rx {
            self.open_ports.insert(port);
        }

        println!("Open Ports for {}:\n{:?}", self.address, self.open_ports);
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

    fn connect_host_list (tx: Sender<u16>) {
        
    }

}
