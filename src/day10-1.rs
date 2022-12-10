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

fn solution() -> Result<i32> {
    let instructions = collect_lines::<Instruction>("input/day10.txt")?;

    let cpu_iter = CpuIterator::new(&instructions);
    Ok(cpu_iter
        .enumerate()
        .filter_map(|(cycle, strength)| is_special(cycle + 2).then_some(strength))
        .sum())
}

struct CpuIterator<'a> {
    cycle: i32,
    x: i32,
    addx: Option<i32>,
    instructions: std::slice::Iter<'a, Instruction>,
}

impl<'a> CpuIterator<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        CpuIterator {
            cycle: 1,
            x: 1,
            addx: None,
            instructions: instructions.iter(),
        }
    }
}

impl Iterator for CpuIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;
        if let Some(addx) = self.addx {
            self.x += addx;
            self.addx = None;
        } else if let Some(instruction) = self.instructions.next() {
            if let Instruction::Addx(addx) = instruction {
                self.addx = Some(*addx);
            }
        } else {
            return None;
        }
        Some(self.cycle * self.x)
    }
}

fn is_special(cycle: usize) -> bool {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split_whitespace();
        match itr.next() {
            Some("noop") => Ok(Self::Noop),
            Some("addx") => Ok(Self::Addx(
                itr.next()
                    .ok_or(anyhow!("addx instruction missing amount!"))?
                    .parse()?,
            )),
            _ => Err(anyhow!("Invalid instruction: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("noop", Instruction::Noop)]
    #[case("addx 3", Instruction::Addx(3))]
    #[case("addx -20", Instruction::Addx(-20))]
    fn instruction_from_str(#[case] input: &str, #[case] expected: Instruction) {
        assert_eq!(input.parse::<Instruction>().unwrap(), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 15120);
    }
}
