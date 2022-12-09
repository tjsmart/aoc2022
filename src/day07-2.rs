use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_lines;
use aoc::time_it;

const TOTAL_SPACE: usize = 70000000;
const MIN_UNUSED_SPACE: usize = 30000000;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let lines = collect_lines::<TerminalLine>("input/day07.txt")?;

    let mut total_sizes: HashMap<String, usize> = HashMap::new();
    let mut curdir = SimplePath::new();

    for line in lines {
        update_size(line, &mut curdir, &mut total_sizes)?;
    }

    while curdir.dirs.len() != 1 {
        update_size(
            TerminalLine::CD("..".to_string()),
            &mut curdir,
            &mut total_sizes,
        )?;
    }

    let space_in_use = total_sizes.get("/").unwrap().clone();
    let space_to_free = MIN_UNUSED_SPACE - (TOTAL_SPACE - space_in_use);

    Ok(total_sizes
        .into_iter()
        .filter_map(|(_, size)| (size > space_to_free).then_some(size))
        .min()
        .unwrap())
}

fn update_size(
    line: TerminalLine,
    curdir: &mut SimplePath,
    total_sizes: &mut HashMap<String, usize>,
) -> Result<()> {
    match line {
        TerminalLine::CD(dir) => {
            if dir == ".." {
                let child_size = total_sizes.get(&curdir.to_string()).unwrap().clone();
                curdir.push(&dir)?;
                let total_size = total_sizes.get_mut(&curdir.to_string()).unwrap();
                *total_size += child_size;
            } else {
                curdir.push(&dir)?;
                total_sizes.insert(curdir.to_string(), 0);
            }
        }
        TerminalLine::File(size, _) => {
            let total_size = total_sizes.get_mut(&curdir.to_string()).unwrap();
            *total_size += size;
        }
        _ => {}
    }
    Ok(())
}

struct SimplePath {
    dirs: Vec<String>,
}

impl SimplePath {
    fn new() -> Self {
        SimplePath { dirs: vec![] }
    }

    fn push(&mut self, s: &str) -> Result<()> {
        match s {
            ".." => {
                let _ = self.dirs.pop().ok_or(anyhow!("Already at root!"))?;
            }
            "/" => self.dirs.push("".to_string()),
            s => self.dirs.push(s.to_string()),
        }

        Ok(())
    }
}

impl ToString for SimplePath {
    fn to_string(&self) -> String {
        self.dirs.join("/").add("/")
    }
}

#[derive(Debug, PartialEq)]
enum TerminalLine {
    CD(String),
    LS,
    Dir(String),
    File(usize, String),
}

impl FromStr for TerminalLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", dest] => Ok(TerminalLine::CD(dest.to_string())),
            ["$", "ls"] => Ok(TerminalLine::LS),
            ["dir", name] => Ok(TerminalLine::Dir(name.to_string())),
            [size, name] => Ok(TerminalLine::File(size.parse()?, name.to_string())),
            _ => Err(anyhow!("Unexpected terminal line: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("$ cd test", TerminalLine::CD("test".to_string()))]
    #[case("$ ls", TerminalLine::LS)]
    #[case("dir foo", TerminalLine::Dir("foo".to_string()))]
    #[case("1234 bar.txt", TerminalLine::File(1234, "bar.txt".to_string()))]
    fn parse_line_tests(#[case] input: &str, #[case] expected: TerminalLine) {
        assert_eq!(input.parse::<TerminalLine>().unwrap(), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 1117448);
    }
}
