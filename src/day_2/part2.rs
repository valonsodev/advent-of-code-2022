use crate::utils::io::iter_lines;
trait Points {
    fn value(&self) -> i32;
    fn printself(&self) -> String;
}
#[derive(PartialEq, Eq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl std::str::FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(format!("'{}' is not a valid value for a Move", s)),
        }
    }
}

#[derive(Debug)]
enum MatchResult {
    Loss,
    Draw,
    Win,
}
impl std::str::FromStr for MatchResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Loss),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(format!("'{}' is not a valid value for a MatchResult", s)),
        }
    }
}
impl Points for Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn printself(&self) -> String {
        format!("{:?}", self)
    }
}
impl Points for MatchResult {
    fn value(&self) -> i32 {
        match *self {
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
            MatchResult::Win => 6,
        }
    }
    fn printself(&self) -> String {
        format!("{:?}", self)
    }
}
pub fn main() {
    let filename = "./data/day2.txt".to_string(); //Relative to top level of project
    let input =
        iter_lines(&filename).unwrap_or_else(|_| panic!("The file {} does not exist", &filename));
    let mut total_score = 0;
    for line in input {
        if let Ok(moves) = line.map(|l| {
            l.split(' ')
                .map(str::to_owned)
                .filter_map(|e| {
                    (e.parse::<MatchResult>()
                        .map(|x| Box::new(x) as Box<dyn Points>))
                    .or_else(|_| e.parse::<Move>().map(|x| Box::new(x) as Box<dyn Points>))
                    .ok()
                })
                .collect::<Vec<Box<dyn Points>>>()
        }) {
            if moves.len() == 2 {
                let enemy_move = &moves[0];
                let match_result = &moves[1];
                let mut round_score = 0;
                log::debug!(
                    "First letter is {}({} points), second letter is {}({} points)",
                    enemy_move.printself(),
                    enemy_move.value(),
                    match_result.printself(),
                    match_result.value()
                );
                let expected_points = match match_result.value() {
                    0 => (enemy_move.value() + 1) % 3 + 1,
                    3 => enemy_move.value(),
                    6 => (enemy_move.value() % 3) + 1,
                    _=> panic!("Somehting is wrong in here"),
                };
                log::debug!("The necessary move to achieve {} against {} has {} points", match_result.printself(), enemy_move.printself(), expected_points);
                round_score += match_result.value() + expected_points;
                total_score += round_score;
            }
        } else {
            log::error!("Invalid line read");
        }

    }
    log::info!("Total game score is {}", total_score);
}
