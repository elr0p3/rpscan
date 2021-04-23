use std::error::Error;
use std::path::Path;


const NUM_FILE_TYPES: usize = 2;
pub const OUTF_NAMES: [&str; NUM_FILE_TYPES] =
    ["outfile_normal", "outfile_grepable"];
pub const OUTF_SHORT: [u8; NUM_FILE_TYPES] =
    [b'N', b'G'];


use super::Output;

pub struct Outfile<'a> {
    mode: u8,
    output: &'a Output<'a>,
    path: &'a str,
}


impl<'a> Outfile<'a> {

    pub fn new (output: &'a Output, mode: u8, path: &'a str) -> Self {
        Self {
            mode,
            output,
            path,
        }
    }


    pub fn is_valid_file (name: &str) -> bool {
        !Path::new(name).exists()
    }
}
