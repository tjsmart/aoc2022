use std::cmp::Ordering;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_lines;
use aoc::time_it;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<u32> {
    let games = collect_lines::<Game>("input/day02.txt")?;

    Ok(games.into_iter().map(|game| game.score()).sum())
}

#[derive(Eq, PartialEq, PartialOrd, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lost,
    Tied,
    Won,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lost),
            "Y" => Ok(Outcome::Tied),
            "Z" => Ok(Outcome::Won),
            _ => Err(anyhow!("Blah!")),
        }
    }
}

impl Outcome {
    fn my_move(&self, their_move: &Move) -> Move {
        match (self, their_move) {
            (Outcome::Lost, Move::Rock) => Move::Scissors,
            (Outcome::Won, Move::Rock) => Move::Paper,

            (Outcome::Lost, Move::Paper) => Move::Rock,
            (Outcome::Won, Move::Paper) => Move::Scissors,

            (Outcome::Lost, Move::Scissors) => Move::Paper,
            (Outcome::Won, Move::Scissors) => Move::Rock,

            (Outcome::Tied, my_move) => my_move.clone(),
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Paper, Move::Rock) => Ordering::Greater,
            (Move::Paper, Move::Scissors) => Ordering::Less,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (Move::Scissors, Move::Paper) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(anyhow!("Blah!")),
        }
    }
}

struct Game {
    theirs: Move,
    outcome: Outcome,
}

impl Game {
    fn score(&self) -> u32 {
        let win_score = match self.outcome {
            Outcome::Lost => 0,
            Outcome::Tied => 3,
            Outcome::Won => 6,
        };

        let my_move = self.outcome.my_move(&self.theirs);

        let my_score = match my_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let score = win_score + my_score;
        score
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (theirs, mine) = s
            .split_once(" ")
            .ok_or(anyhow!("Line missing space delimiter!"))?;

        Ok(Game {
            theirs: theirs.parse()?,
            outcome: mine.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert_eq!(Game::from_str("A X").unwrap().score(), 0 + 3); // rock lost
        assert_eq!(Game::from_str("A Y").unwrap().score(), 3 + 1); // rock tied
        assert_eq!(Game::from_str("A Z").unwrap().score(), 6 + 2); // rock won

        assert_eq!(Game::from_str("B X").unwrap().score(), 0 + 1);
        assert_eq!(Game::from_str("B Y").unwrap().score(), 3 + 2);
        assert_eq!(Game::from_str("B Z").unwrap().score(), 6 + 3);

        assert_eq!(Game::from_str("C X").unwrap().score(), 0 + 2);
        assert_eq!(Game::from_str("C Y").unwrap().score(), 3 + 3);
        assert_eq!(Game::from_str("C Z").unwrap().score(), 6 + 1);
    }

    #[test]
    fn sln2() {
        assert_eq!(solution().unwrap(), 14610);
    }
}
