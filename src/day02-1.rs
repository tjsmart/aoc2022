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
    mine: Move,
}

impl Game {
    fn score(&self) -> u32 {
        let my_score = match self.mine {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let win_score = match self.mine.cmp(&self.theirs) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
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
            mine: mine.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert_eq!(Game::from_str("A X").unwrap().score(), 3 + 1); // rock rock
        assert_eq!(Game::from_str("A Y").unwrap().score(), 6 + 2); // rock paper
        assert_eq!(Game::from_str("A Z").unwrap().score(), 0 + 3); // rock scissors

        assert_eq!(Game::from_str("B X").unwrap().score(), 0 + 1);
        assert_eq!(Game::from_str("B Y").unwrap().score(), 3 + 2);
        assert_eq!(Game::from_str("B Z").unwrap().score(), 6 + 3);

        assert_eq!(Game::from_str("C X").unwrap().score(), 6 + 1);
        assert_eq!(Game::from_str("C Y").unwrap().score(), 0 + 2);
        assert_eq!(Game::from_str("C Z").unwrap().score(), 3 + 3);
    }

    #[test]
    fn sln1() {
        assert_eq!(solution().unwrap(), 9241);
    }
}
