use std::collections::HashSet;
use std::str::FromStr;
// use std::thread;
// use std::time;
// use std::vec;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_lines;
use aoc::time_it;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

type Coord = (i32, i32);

fn solution() -> Result<usize> {
    let motions = collect_lines::<Motion>("input/day09.txt")?;
    let mut rope = Rope::default();
    let mut tail_coords: HashSet<Coord> = HashSet::new();
    // let ten_millis = time::Duration::from_millis(20);

    for motion in motions {
        for _ in 0..motion.count {
            rope.wiggle(&motion.dir);
            tail_coords.insert(rope.tail());
        }
        // _draw(&rope);
        // thread::sleep(ten_millis);
    }

    Ok(tail_coords.len())
}

// fn _draw(rope: &Rope) {
//     print!("{esc}c", esc = 27 as char);
//     let mut screen = vec![vec![".".to_string(); 100]; 120];
//
//     for (i, knot) in rope.knots.iter().enumerate() {
//         let y = (knot.0 + 80) as usize;
//         let x = (knot.1 + 80) as usize;
//         screen[y][x] = i.to_string();
//     }
//
//     for col in 0..120 {
//         println!("{}", screen[col].join(" "));
//     }
// }

#[derive(Default)]
struct Rope {
    knots: [Coord; 10],
}

impl Rope {
    fn wiggle(&mut self, dir: &Direction) {
        self.move_head(dir);
        self.update_tails();
    }

    fn tail(&self) -> Coord {
        self.knots[self.knots.len() - 1]
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                self.knots[0].0 += 1;
            }
            Direction::Down => {
                self.knots[0].0 -= 1;
            }
            Direction::Left => {
                self.knots[0].1 -= 1;
            }
            Direction::Right => {
                self.knots[0].1 += 1;
            }
        }
    }

    fn update_tails(&mut self) {
        for i in 1..self.knots.len() {
            if (self.knots[i - 1].0 - self.knots[i].0).abs() == 2
                || (self.knots[i - 1].1 - self.knots[i].1).abs() == 2
            {
                self.knots[i].0 += (self.knots[i - 1].0 - self.knots[i].0).signum();
                self.knots[i].1 += (self.knots[i - 1].1 - self.knots[i].1).signum();
            }
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
        assert_eq!(solution().unwrap(), 2386);
    }
}
