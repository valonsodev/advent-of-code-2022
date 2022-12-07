use crate::utils::calories::group_calories;
use crate::utils::io::iter_lines;

use std::collections::BinaryHeap;
pub fn main() {
    let filename = "./data/day1.txt".to_string(); //Relative to top level of project

    let test =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut christmas_creatures = BinaryHeap::new();
    let mut last_elf_calories = 0;

    for line in test {
        if let Ok(valid_line) = line.map(|sex| sex.trim().to_string()) {
            if valid_line.is_empty() {
                log::debug!("New elf");
                christmas_creatures.push(last_elf_calories);
                last_elf_calories = 0;
            } else if let Ok(calories) = valid_line.trim().parse::<u32>() {
                last_elf_calories += calories;
            } else {
                log::error!("Invalid line: {}", valid_line);
            }
        } else {
            log::error!("Uh oh...");
        }
    }
    for (rank, elf) in christmas_creatures
        .clone()
        .into_sorted_vec()
        .iter()
        .rev()
        .enumerate()
    {
        log::debug!("Elf number {} has {} calories", rank + 1, elf);
    }
    log::info!(
        "The top 3 elfs have {} calories",
        group_calories(christmas_creatures.clone().into_vec(), vec![0, 1, 2])
    );
}
