pub fn group_calories(ranking: Vec<u32>, group: Vec<usize>) -> u32 {
    return group
        .iter()
        .map(|c| (ranking.get(*c).unwrap_or(&0)))
        .sum();
}
