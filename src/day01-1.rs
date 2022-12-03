use anyhow::Result;
use aoc::collect_lines;
use aoc::time_it;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<u32> {
    let data = collect_lines::<String>("input/day01.txt")?;

    let mut max = 0u32;
    let mut sum = 0u32;

    for x in data {
        if x.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum = sum + x.parse::<u32>()?;
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 72511);
    }
}
