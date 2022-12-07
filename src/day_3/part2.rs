use std::collections::HashSet;


use crate::utils::{io::iter_lines, priorities::priority};

pub fn main() {
    let filename = "./data/day3.txt".to_string(); //Relative to top level of project

    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut total_priorities = 0;
    let mut buffer: Vec<Vec<char>> = vec![];
    for line in input {
        if let Ok(mut line_chars) = line.map(|l| l.trim().to_string().chars().collect::<Vec<char>>()) {
            line_chars.sort();
            line_chars.dedup();
            buffer.push(line_chars);
            if buffer.len() == 3 {
                let mut common_chars = HashSet::from_iter(buffer[0].iter().copied());
                for (i, vec) in buffer.iter().enumerate() {
                    common_chars = vec
                        .iter()
                        .filter(|e| !common_chars.insert(**e))
                        .copied()
                        .collect::<HashSet<char>>();

                    log::debug!("Common chars in first {} lines are {:?}", i+1, common_chars);
                }
                for i in common_chars.iter(){
                    total_priorities+= priority(i);
                }
                buffer.clear();
            }
        } else {
            log::error!("Uh oh...");
        }
    }
    log::info!("Sum of priorities is {}", total_priorities);
}

