use std::fmt::{Display, Formatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./aoc-2022/inputs/02")?;
    println!("One: {:?}", part_one(&input)?);
    println!("Two: {:?}", part_two(&input)?);
    Ok(())
}

fn part_one(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    input.split('\n').map(|round| score_a_round_one(round)).sum()
}

fn part_two(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    input.split('\n').map(|round| score_a_round_two(round)).sum()
}

/// Given a round in the format:
///
/// "_ _"
///
/// Return how many points we got for that round. The first letter is one of "A" (Rock), "B"
/// (Paper), or "C" (Scissors). The second letter is one of "X" (Rock), "Y" (Paper), or "Z"
/// (Scissors). Anything else is invalid.
///
/// Expected points are a combination of whether or not we won, plus some specific points for our
/// choices. We get 6 points for a win, 3 points for a draw, and 0 points for a loss. We
/// additionally get 1 point for playing Rock, 2 points for playing Paper, and 3 points for
/// playing Scissors
fn score_a_round_one(entry: &str) -> Result<i32, Box<dyn std::error::Error>> {
    match entry {
        "A X" => Ok(3 + 1), // Their Rock     ties my     Rock,     1 point  for Rock
        "A Y" => Ok(6 + 2), // Their Rock     loses to my Paper,    2 points for Paper
        "A Z" => Ok(0 + 3), // Their Rock     beats my    Scissors, 3 points for Scissors
        "B X" => Ok(0 + 1), //       Paper                Rock,     1            Rock
        "B Y" => Ok(3 + 2), //       Paper                Paper,    2            Paper
        "B Z" => Ok(6 + 3), //       Paper                Scissors, 3            Scissors
        "C X" => Ok(6 + 1), //       Scissors             Rock,     1            Rock
        "C Y" => Ok(0 + 2), //       Scissors             Paper,    2            Paper
        "C Z" => Ok(3 + 3), //       Scissors             Scissors, 3            Scissors
        _ => Err(Box::from(MyError {}))
    }
}

/// Given a round in the format:
///
/// "_ _"
///
/// Return how many points we got for that round. The first letter is one of "A" (Rock), "B"
/// (Paper), or "C" (Scissors). The second letter is one of "X" (Lose), "Y" (Draw), or "Z" (Win).
/// Anything else is invalid.
///
/// Expected points are a combination of whether or not we won, plus some specific points for our
/// choices. We get 6 points for a win, 3 points for a draw, and 0 points for a loss. We
/// additionally get 1 point for playing Rock, 2 points for playing Paper, and 3 points for
/// playing Scissors
fn score_a_round_two(entry: &str) -> Result<i32, Box<dyn std::error::Error>> {
    match entry {
        "A X" => Ok(0 + 3), // We Lose, by playing Scissors against their Rock
        "A Y" => Ok(3 + 1), // We Draw, by playing Rock     against their Rock
        "A Z" => Ok(6 + 2), // We Win,  by playing Paper    against their Rock
        "B X" => Ok(0 + 1), //    Lose,            Rock                   Paper
        "B Y" => Ok(3 + 2), //    Draw,            Paper                  Paper
        "B Z" => Ok(6 + 3), //    Win,             Scissors               Paper
        "C X" => Ok(0 + 2), //    Lose,            Paper                  Scissors
        "C Y" => Ok(3 + 3), //    Draw,            Scissors               Scissors
        "C Z" => Ok(6 + 1), //    Win,             Rock                   Scissors
        _ => Err(Box::from(MyError {}))
    }
}


#[derive(Debug)]
struct MyError {}

impl std::error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        panic!("Just don't call me")
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_one() -> Result<(), Box<dyn std::error::Error>> {
        // The current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/02_test")?;
        assert_eq!(part_one(&*test_input)?, 15);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn std::error::Error>> {
        // The current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/02_test")?;
        assert_eq!(part_one(&*test_input)?, 12);
        Ok(())
    }
}