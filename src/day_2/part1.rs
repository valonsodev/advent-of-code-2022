use crate::utils::io::iter_lines;
trait Points {
    fn value(&self) -> i32;
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
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("'{}' is not a valid value for a Move", s)),
        }
    }
}
impl Move {
    fn face(&self, against: &Self) -> MatchResult {
        match self {
            _ if self == against => MatchResult::Draw,
            Move::Rock => {
                if let Move::Scissors = against {
                    MatchResult::Win
                } else {
                    MatchResult::Loss
                }
            }
            Move::Paper => {
                if let Move::Rock = against {
                    MatchResult::Win
                } else {
                    MatchResult::Loss
                }
            }
            Move::Scissors => {
                if let Move::Paper = against {
                    MatchResult::Win
                } else {
                    MatchResult::Loss
                }
            }
        }
    }
}
#[derive(Debug)]
enum MatchResult {
    Loss,
    Draw,
    Win,
}
impl Points for Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
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
                .filter_map(|e| e.parse::<Move>().ok())
                .collect::<Vec<Move>>()
        }) {
            if moves.len() == 2 {
                let enemy_move = &moves[0];
                let player_move = &moves[1];
                let match_result = player_move.face(enemy_move);
                let round_score = match_result.value() + player_move.value();

                log::debug!(
                    "The result of playing {:?}({}) against {:?}({}) is a {:?}({}) = {}",
                    player_move,
                    player_move.value(),
                    enemy_move,
                    enemy_move.value(),
                    match_result,
                    match_result.value(),
                    round_score
                );
                total_score += round_score;
            }
        } else {
            log::error!("Invalid line read");
        }
    }
    log::info!("Total game score is {}", total_score);
}
