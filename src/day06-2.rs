use std::{collections::HashSet, fs};

use anyhow::Result;

use aoc::time_it;
use itertools::Itertools;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let input = fs::read_to_string("input/day06.txt")?;
    Ok(find_marker(&input))
}

type Window<'a> = HashSet<&'a char>;
const WINDOW_SIZE: usize = 14;

fn find_marker(s: &str) -> usize {
    s.chars()
        .collect_vec()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_map(|(idx, chars)| {
            if Window::from_iter(chars.iter()).len() == WINDOW_SIZE {
                Some(idx + WINDOW_SIZE)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn find_marker_tests(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, find_marker(input))
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 3298);
    }
}
