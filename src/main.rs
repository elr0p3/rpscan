use std::env;
use std::process;

mod input_port_data;
use input_port_data::data::PortData;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let data: PortData = match PortData::new(&args) {
        Ok(dt) => dt,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    println!("{:?}", data);
}
