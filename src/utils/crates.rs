pub fn print_tower(tower: &Vec<Vec<char>>) -> String {
    let mut tower_str = "\n".to_string();
    let column_height = tower.iter().fold(0, |accum, item| {
        if accum >= item.len() {
            accum
        } else {
            item.len()
        }
    }) - 1;
    for i in (0..column_height).rev() {
        let mut row_str = "".to_string();
        for column in tower {
            row_str.push_str(
                column
                    .get(i)
                    .map(|c| format!("[{}]  ", c))
                    .unwrap_or_else(|| "     ".to_string())
                    .as_str(),
            );
        }
        row_str.push('\n');
        tower_str.push_str(row_str.as_str());
    }
    for i in 0..tower.len() {
        tower_str.push_str(format!(" {}   ", i + 1).as_str());
    }
    log::debug!("The tower height is {}", column_height);
    log::debug!("The tower width is {}", tower.len());

    tower_str
}
