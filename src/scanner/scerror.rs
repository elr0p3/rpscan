use std::{
    error,
    fmt
};


#[derive(Debug, Clone)]
pub enum ScannerError {
    IpParse,
    RangePortParse,
    SinglePortParse
}


impl error::Error for ScannerError {}


impl fmt::Display for ScannerError {

    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scanner error")
    }

}
