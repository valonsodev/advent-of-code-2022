use crate::utils::io::iter_lines;

pub fn main() {
    let filename = "./data/day4.txt".to_string(); //Relative to top level of project

    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut total_containments = 0;
    for line in input {
        if let Ok(valid_line) = line.map(|l| {
            l.trim()
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<String>>()
        }) {
            if valid_line.len() == 2 {
                let (first_schedule, second_schedule) = (
                    &(valid_line[0]
                        .split('-')
                        .map(str::to_owned)
                        .filter_map(|n| n.parse::<i32>().ok())
                        .collect::<Vec<i32>>())[0..2],
                    &(valid_line[1]
                        .split('-')
                        .map(str::to_owned)
                        .filter_map(|n| n.parse::<i32>().ok())
                        .collect::<Vec<i32>>())[0..2],
                );
                let fc = first_schedule[0] <= second_schedule[0];
                let sc = first_schedule[1] >= second_schedule[1];
                let tc = second_schedule[0] <= first_schedule[0];
                let cc = second_schedule[1] >= first_schedule[1];
                if fc == sc || tc==cc {
                    total_containments+=1;
                    log::debug!(
                        "It appears something contains something {:?} {:?}",
                        first_schedule,
                        second_schedule
                    )
                }
            }
        } else {
            log::error!("Uh oh...");
        }
    }
    log::info!("Total containments {}",total_containments);
}
