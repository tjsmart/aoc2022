use std::cmp::Ordering;
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
        .into_iter()
        .enumerate()
        .map(|(i, sack)| chr_to_priority(sack.get_common().expect(&i.to_string())))
        .sum())
}

fn chr_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) + 1 - ('a' as u32)
    } else {
        (c as u32) + 27 - ('A' as u32)
    }
}

struct RuckSack {
    cmp1: HashSet<char>,
    cmp2: HashSet<char>,
}

impl RuckSack {
    fn get_common(&self) -> Result<char> {
        Ok(self
            .cmp1
            .intersection(&self.cmp2)
            .next()
            .ok_or(anyhow!("No matching item!"))?
            .to_owned())
    }
}

impl FromStr for RuckSack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        let mut cmp1 = HashSet::new();
        let mut cmp2 = HashSet::new();
        for (i, c) in s.chars().enumerate() {
            match i.cmp(&mid) {
                Ordering::Less => cmp1.insert(c),
                _ => cmp2.insert(c),
            };
        }

        Ok(RuckSack { cmp1, cmp2 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case(s: &str, exp1: &str, exp2: &str) {
        let sack = RuckSack::from_str(s).unwrap();
        assert_eq!(sack.cmp1, HashSet::from_iter(exp1.chars()));
        assert_eq!(sack.cmp2, HashSet::from_iter(exp2.chars()));
    }

    #[test]
    fn cmp() {
        test_case("abcd", "ab", "cd");
        test_case("abbcdd", "ab", "cd");
        test_case("bbcc", "b", "c");
    }

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
