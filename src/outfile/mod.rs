// use std::error::Error;


const NORMAL: u8 = b'N';


pub struct Outfile {
    mode: char,
    ports: Vec<u16>,
}


impl Outfile {

    pub fn new (mode: &[u8]) -> Result<Self, &'static str> {
        let mode = match mode[0] {
            NORMAL => NORMAL as char,
            _ => return Err("tonto tonto puto tonto"),
        };

        Ok(Self {
            mode,
            ports: vec![],
        })
    }

}
