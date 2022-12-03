use std::collections::HashSet;
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
    let sacks = collect_lines::<RuckSack>("input/day03_realdeal.txt")?;
    Ok(sacks
        .chunks(3)
        .map(|sacks| chr_to_priority(get_common(&sacks[0], &sacks[1], &sacks[2]).expect("what?")))
        .sum())
}

fn chr_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) + 1 - ('a' as u32)
    } else {
        (c as u32) + 27 - ('A' as u32)
    }
}

#[derive(Debug)]
struct RuckSack {
    cmp: HashSet<char>,
}

fn get_common(sack1: &RuckSack, sack2: &RuckSack, sack3: &RuckSack) -> Result<char> {
    Ok(sack1
        .cmp
        .intersection(&sack2.cmp)
        .find(|c| sack3.cmp.contains(c))
        .ok_or(anyhow!("No matching item!"))?
        .to_owned())
}

impl FromStr for RuckSack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RuckSack {
            cmp: HashSet::from_iter(s.chars()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chr_conv() {
        assert_eq!(chr_to_priority('a'), 1);
        assert_eq!(chr_to_priority('b'), 2);
        assert_eq!(chr_to_priority('c'), 3);
        assert_eq!(chr_to_priority('z'), 26);
        assert_eq!(chr_to_priority('A'), 27);
        assert_eq!(chr_to_priority('B'), 28);
        assert_eq!(chr_to_priority('C'), 29);
        assert_eq!(chr_to_priority('Z'), 52);
    }

    // #[test]
    // fn sln1() {
    //     assert_eq!(solution().unwrap(), 9241);
    // }

    // #[test]
    // fn sln2() {
    //     assert_eq!(solution(3).unwrap(), 212117);
    // }
}
