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

fn solution() -> Result<usize> {
    let motions = collect_lines::<Motion>("input/day09.txt")?;
    let mut rope = Rope::default();
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for motion in motions {
        for _ in 0..motion.count {
            rope.wiggle(&motion.dir);
            tail_positions.insert(rope.tail);
        }
    }

    Ok(tail_positions.len())
}

type Coord = (i32, i32);

#[derive(Default)]
struct Rope {
    head: Coord,
    tail: Coord,
}

impl Rope {
    fn wiggle(&mut self, dir: &Direction) {
        self.move_head(dir);
        self.update_tail();
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                self.head.0 += 1;
            }
            Direction::Down => {
                self.head.0 -= 1;
            }
            Direction::Left => {
                self.head.1 -= 1;
            }
            Direction::Right => {
                self.head.1 += 1;
            }
        }
    }

    fn update_tail(&mut self) {
        if (self.head.0 - self.tail.0).abs() == 2 || (self.head.1 - self.tail.1).abs() == 2 {
            self.tail.0 += (self.head.0 - self.tail.0).signum();
            self.tail.1 += (self.head.1 - self.tail.1).signum();
        }
    }
}

#[derive(Debug)]
struct Motion {
    dir: Direction,
    count: usize,
}

impl FromStr for Motion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s
            .split_once(" ")
            .ok_or(anyhow!("Missing delimeter in line!"))?;
        Ok(Motion {
            dir: dir.parse()?,
            count: count.parse()?,
        })
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Unrecognized direction: '{}'", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 6181);
    }
}
