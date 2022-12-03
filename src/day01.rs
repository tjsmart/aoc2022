use anyhow::Result;
use aoc::collect_blocks;
use aoc::time_it;
use itertools::sorted;

fn main() -> Result<()> {
    time_it(|| solution(1))?;
    time_it(|| solution(3))?;
    Ok(())
}

fn solution(take: usize) -> Result<u32> {
    let blocks = collect_blocks::<u32>("input/day01_realdeal.txt")?;

    let block_sums = blocks
        .into_iter()
        .map(|block| block.into_iter().sum())
        .collect::<Vec<u32>>();

    Ok(sorted(block_sums).rev().take(take).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln1() {
        assert_eq!(solution(1).unwrap(), 72511);
    }

    #[test]
    fn sln2() {
        assert_eq!(solution(3).unwrap(), 212117);
    }
}
