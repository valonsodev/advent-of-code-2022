use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn iter_lines(filename: &impl AsRef<Path>) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_file_string(filepath: &String) -> io::Result<String> {
    read_to_string(filepath)
}
