
use crate::utils::io::iter_lines;

pub fn main() {
    let filename = "./data/day.txt".to_string(); //Relative to top level of project

    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    for line in input {
        if let Ok(valid_line) = line.map(|l| l.trim().to_string()) {

        } else {
            log::error!("Uh oh...");
        }
    }
    log::info!("");
}

