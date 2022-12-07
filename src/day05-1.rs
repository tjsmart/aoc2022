use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_blocks;
use aoc::time_it;
use itertools::Itertools;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<String> {
    let blocks = collect_blocks::<String>("input/day05.txt")?;
    assert_eq!(blocks.len(), 2);

    let mut stacks = parse_stacks(&blocks[0]);
    let moves = parse_moves(&blocks[1])?;

    for mv in moves {
        for _ in 0..mv.count {
            match stacks[mv.from - 1].pop() {
                Some(x) => stacks[mv.to - 1].push(x),
                None => panic!("Whoopsies that stack is empty now!"),
            };
        }
    }

    Ok(stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>())
}

fn parse_moves(s: &[String]) -> Result<Vec<Move>> {
    s.iter().map(|mv| mv.parse()).collect()
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // example: move 1 from 2 to 1
        if let Some((count, from, to)) = s
            .split_whitespace()
            .filter_map(|digit| match digit.parse::<usize>() {
                Ok(digit) => Some(digit),
                _ => None,
            })
            .collect_tuple()
        {
            Ok(Move { count, from, to })
        } else {
            Err(anyhow!("Failed to parse move string: {}", s))
        }
    }
}

type Stacks = Vec<Vec<char>>;

fn parse_stacks(s: &[String]) -> Stacks {
    let (stack_idxs, stack_contents) = s.split_last().unwrap();

    let stack_idxs = stack_idxs
        .chars()
        .enumerate()
        .filter_map(|(char_idx, stack_num)| match stack_num.to_digit(10) {
            Some(_) => Some(char_idx),
            None => None,
        })
        .collect::<Vec<_>>();

    stack_idxs
        .into_iter()
        .map(|stack_idx| {
            stack_contents
                .iter()
                .rev()
                .map_while(|line| {
                    let x = line.chars().nth(stack_idx).unwrap();
                    (x != ' ').then_some(x)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), "QNNTGTPFN");
    }
}
