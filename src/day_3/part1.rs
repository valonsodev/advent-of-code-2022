use std::collections::HashSet;


use crate::utils::{io::iter_lines, priorities::priority};

pub fn main() {
    let filename = "./data/day3.txt".to_string(); //Relative to top level of project

    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut total_priorities = 0;
    for line in input {
        if let Ok(line_chars) =
            line.map(|l| l.trim().to_string().chars().collect::<Vec<char>>())
        {
            let mut first_hash = HashSet::new();
            let (first_half, second_half) = line_chars.split_at(line_chars.len() / 2);
            for ele in first_half.iter() {
                first_hash.insert(*ele);
            }
            let wrong_second = second_half
                .iter()
                .filter(|e| first_hash.contains(&**e))
                .copied()
                .collect::<HashSet<char>>();
            log::debug!("{:?}", line_chars);
            for i in wrong_second.iter() {
                let char_priority = priority(i);
                total_priorities+=char_priority;
            }
        } else {
            log::error!("Uh oh...");
        }
    }
    log::info!("Sum of priorities is {}",total_priorities);
}
