use crate::utils::io::read_file_string;
use crate::utils::marker::find_marker;

pub fn main() {
    let filename = "./data/day6.txt".to_string(); //Relative to top level of project

    let input = read_file_string(&filename)
        .unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
        let index = find_marker(input.as_str(),4);

    log::info!(
        "The first time a start-of-packet marker appears is after character nº {}",
        index.expect("Marker not found")
    );
}
