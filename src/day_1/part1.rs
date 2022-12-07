
use crate::utils::io::iter_lines;

pub fn main() {
    let filename = "./data/day1.txt".to_string(); //Relative to top level of project

    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut fattest_elf = (0, 0);
    let mut last_elf_calories = 0;
    let mut current_elf = 0;
    for line in input {
        if let Ok(valid_line) = line.map(|l| l.trim().to_string()) {
            if valid_line.is_empty() {
                log::debug!("New elf");
                if last_elf_calories > fattest_elf.1 {
                    fattest_elf.1 = last_elf_calories;
                    fattest_elf.0 = current_elf;
                }
                last_elf_calories = 0;
                current_elf += 1;
            } else if let Ok(calories) = valid_line.trim().parse::<u32>() {
                last_elf_calories += calories;
            } else {
                log::error!("Invalid line: {}", valid_line);
            }
        } else {
            log::error!("Uh oh...");
        }
    }
    log::info!(
        "The fattest elf is number {} with {} calories",
        fattest_elf.0, fattest_elf.1
    );
}

