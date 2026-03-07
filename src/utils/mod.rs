use std::fs;
use std::path::Path;
use std::io;

pub fn read_source_file<P: AsRef<Path>>(path : P) -> io::Result<String>{
    fs::read_to_string(path)
}