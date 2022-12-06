use std::ops::Range;
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

fn solution() -> Result<usize> {
    let pairs = collect_lines::<Pair>("input/day04.txt")?;
    Ok(pairs.into_iter().filter(|pair| pair.overlap()).count())
}

struct Pair {
    r1: Range<u32>,
    r2: Range<u32>,
}

trait Overlap {
    fn overlap(&self) -> bool;
}

impl Overlap for Pair {
    fn overlap(&self) -> bool {
        self.r1.clone().any(|x| self.r2.contains(&x))
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s
            .trim()
            .split_once(',')
            .ok_or(anyhow!("Pair does not contain a comma!"))?;
        Ok(Pair {
            r1: parse_range(r1)?,
            r2: parse_range(r2)?,
        })
    }
}

fn parse_range(s: &str) -> Result<Range<u32>> {
    let (start, end) = s
        .split_once('-')
        .ok_or(anyhow!("Range does not contain a dash!"))?;
    Ok(start.parse()?..(end.parse::<u32>()? + 1u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        assert_eq!(Pair::from_str("2-4,6-8").unwrap().overlap(), false);
        assert_eq!(Pair::from_str("2-3,4-5").unwrap().overlap(), false);
        assert_eq!(Pair::from_str("5-7,7-9").unwrap().overlap(), true);
        assert_eq!(Pair::from_str("2-8,3-7").unwrap().overlap(), true);
        assert_eq!(Pair::from_str("6-6,4-6").unwrap().overlap(), true);
        assert_eq!(Pair::from_str("2-6,4-8").unwrap().overlap(), true);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 911);
    }
}
