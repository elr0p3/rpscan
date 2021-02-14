
pub mod data {
    use std::net::{TcpStream, Ipv4Addr};
    
    const NOT_IP: &'static str = "ERROR! IP address not introduced";
    const WRONG_IP: &'static str = "ERROR! IP introduced is not valid";
    const FEW_ARGS: &'static str = "ERROR! Not enough arguments introduced";
    const NOT_THREADS: &'static str = "ERROR! Number of thread not found";

    const F_ADDRESS: &'static str = "-a";
    const F_THREADS: &'static str = "-t";
    pub const F_HELP: &'static str= "-h";


    #[derive(Debug)]
    pub struct PortData {
        ip_addr: Ipv4Addr,
        thread_num: u16,
        start_port: u16,
        last_port: u16,
    }

    impl PortData {
        
        pub fn new(args: &[String]) -> Result<Self, &'static str> {
            if args.len() < 1 {
                return Err(NOT_IP); 
            }

            if args.contains(&F_HELP.to_string()) {
                Self::print_help();
                return Err(F_HELP);
            }

            let start_port: u16 = u16::MIN;
            let last_port: u16 = u16::MAX;

            let index: usize = match args.iter().position(|r| r == F_ADDRESS) {
                Some(index) => index + 1,
                None => 0,
            };
            let ip_addr: Ipv4Addr = match Self::parse_string_to_ip(args, index) {
                Ok(addr) => addr,
                Err(err) => return Err(err),
            };

            let thread_num: u16 = match args.iter().position(|r| r == F_THREADS) {
                Some(index) => {
                    match args.get(index + 1).unwrap_or(&"1".to_string()).parse::<u16>() {
                        Ok(num) => num,
                        Err(_) => return Err(NOT_THREADS),
                    }
                    // args.get(index + 1)
                        // .unwrap_or(&"1".to_string())
                        // .parse::<u16>()
                        // .unwrap_or(1)
                },
                None => 1,
            };

            Ok(Self{
                ip_addr,
                thread_num,
                start_port,
                last_port,
            })
        }

        fn print_help() {
            print!(r#"
HELP MENU:

 -a     IP address (IPv4 for the moment)
 -t     Number of threads you want to use
 -h     Display this menu

Sample:
cargo r 192.168.1.237
"#);
        }

        // fn test(x: String) -> Result<i32, &'static str> {
            // Ok(x.parse::<i32>().unwrap_or_else(|_| "UwU"))
        // }

        fn parse_string_to_ip(args: &[String], index: usize) -> Result<Ipv4Addr, &'static str> {
            const IP_PARTS: usize = 4;
            let ip_str: String = args[index].clone();
            let ip_splited: Vec<&str> = ip_str.split('.').collect();
            let num_parts: usize = ip_splited.len();

            if num_parts != IP_PARTS {
                Err(WRONG_IP)
            } else {
                let a = ip_splited[0].parse::<u8>().unwrap();
                let b = ip_splited[1].parse::<u8>().unwrap();
                let c = ip_splited[2].parse::<u8>().unwrap();
                let d = ip_splited[3].parse::<u8>().unwrap();
                Ok(Ipv4Addr::new(a, b, c, d))
            }
        }

        pub fn scan(&self) {

        }

    }

}
/*
            let ip_addr: Ipv4Addr = match args.iter().position(|r| r == "-p") {
                Some(index) => {
                    match Self::parse_string_to_ip(args, index + 1) {
                        Ok(addr) => addr,
                        Err(err) => { return Err(err); }
                    }
                },
                None => {
                    let addr_result = match Self::parse_string_to_ip(args, 0) {
                        Ok(addr) => addr,
                        Err(err) => { return Err(err); }
                    };
                    addr_result
                },
            };
*/
