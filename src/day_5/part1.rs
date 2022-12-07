use crate::utils::{io::iter_lines, crates::print_tower};

pub fn main() {
    let filename = "./data/day5.txt".to_string(); //Relative to top level of project

    let mut input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut crate_buffer: Vec<String> = Vec::with_capacity(10);
    // let mut line = input.next();
    while let Ok(x) = input.next().unwrap() {
        if x.is_empty() {
            break;
        }
        crate_buffer.push(x); //.split_whitespace().map(str::to_owned).collect::<Vec<String>>()
    }
    log::debug!("The current crate buffer contains {:?}", crate_buffer);

    let towers_index = crate_buffer
        .pop()
        .expect("It appears there are no towers")
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<String>>();
    let mut towers_of_doom: Vec<Vec<char>> = vec![];
    for _ in 0..towers_index.len() {
        towers_of_doom.push(Vec::with_capacity(crate_buffer.len()))
    }
    for crates_line in crate_buffer.iter().rev() {
        //Vertical iteration
        for i in 0..towers_of_doom.len() {
            let char_crate = crates_line.as_bytes()[1 + i * 4];
            if char_crate.is_ascii_alphabetic() {
                towers_of_doom
                    .get_mut(i)
                    .expect("What")
                    .push(char_crate as char);
            }
        }
    }
    log::debug!("The towers start as {}", print_tower(&towers_of_doom));

    // log::debug!("The next line is {:?}", input.peekable().peek()); //Iterator continues as expected
    for line in input {
        if let Ok(valid_line) = line.map(|l| l.trim().to_string()) {
            let statements = valid_line.split_whitespace().collect::<Vec<_>>();
            let amount_moved = statements[1]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Cant convert statement to num in line {}", valid_line));
            let from_tower = statements[3]
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Cant convert statement to num in line {}", valid_line))
                - 1;
            let to_tower = statements[5]
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Cant convert statement to num in line {}", valid_line))
                - 1;
            for _ in 0..amount_moved {
                log::debug!(
                    "Attempting to move crate from tower {} to tower number {}",
                    from_tower + 1,
                    to_tower + 1
                );

                let popped_crate = towers_of_doom
                    .get_mut(from_tower)
                    .expect("Uh oh")
                    .pop()
                    .expect("Something is wrong with the moves");
                towers_of_doom
                    .get_mut(to_tower)
                    .expect("Invalid tower index")
                    .push(popped_crate);
            }
            // Double mutable reference fails as understood
            // let from_tower= towers_of_doom.get_mut(statements[3].parse::<usize>().expect(format!("Cant convert statement to num in line {}",valid_line).as_str())).expect("Invalid tower index");
            // let to_tower = towers_of_doom.get_mut(statements[5].parse::<usize>().expect(format!("Cant convert statement to num in line {}",valid_line).as_str())).expect("Invalid tower index");
            // for i in 0..amount_moved{
            //     to_tower.push(from_tower.pop().expect("Something is wrong with the instructions"));
            // }
            log::debug!(
                "The state after the line {} is {}",
                valid_line,
                print_tower(&towers_of_doom)
            )
        } else {
            log::error!("Uh oh...");
        }
    }
    let mut code = "".to_string();
    for mut col in towers_of_doom {
        code.push(col.pop().unwrap_or(' '))
    }
    log::info!("The crate code is {}", code);
}
