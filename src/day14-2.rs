use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
// use std::thread;
// use std::time;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_lines;
use aoc::time_it;

// const SLEEP: time::Duration = time::Duration::from_millis(30);

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let rock_paths = collect_lines::<RockPath>("input/day14.txt")?;
    let mut cave = Cave::new();

    for path in rock_paths.into_iter() {
        cave.fill(path);
    }

    cave.add_floor();

    let num_rock = cave.grid.len();

    while cave.drop_sand() {
        // print!("{esc}c", esc = 27 as char);
        // thread::sleep(SLEEP);
        // println!("{cave}")
    }

    Ok(cave.grid.len() - num_rock)
}

#[derive(Debug)]
enum Fill {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Cave {
    grid: HashMap<Point, Fill>,
    bottom: i32,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.grid.keys().map(|p| p.x).min().unwrap();
        let max_x = self.grid.keys().map(|p| p.x).max().unwrap();

        for y in 0..=self.bottom {
            let line: String = (min_x..=max_x)
                .map(|x| match self.grid.get(&Point::new(x, y)) {
                    Some(Fill::Sand) => 'o',
                    Some(Fill::Rock) => '#',
                    None => '.',
                })
                .collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl Cave {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bottom: i32::min_value(),
        }
    }

    fn add_floor(&mut self) {
        self.bottom += 2;
        for x in (499 - self.bottom)..=(501 + self.bottom) {
            self.grid.insert(Point::new(x, self.bottom), Fill::Rock);
        }
    }

    fn fill(&mut self, path: RockPath) {
        for point in path.into_iter() {
            (point.y > self.bottom).then(|| self.bottom = point.y);
            self.grid.insert(point, Fill::Rock);
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = Point::new(500, 0);

        if self.grid.contains_key(&sand) {
            return false;
        }

        while sand.y < self.bottom {
            if !self.grid.contains_key(&Point::new(sand.x, sand.y + 1)) {
                sand.y += 1;
            } else if !self.grid.contains_key(&Point::new(sand.x - 1, sand.y + 1)) {
                sand.x -= 1;
                sand.y += 1;
            } else if !self.grid.contains_key(&Point::new(sand.x + 1, sand.y + 1)) {
                sand.x += 1;
                sand.y += 1;
            } else {
                self.grid.insert(sand, Fill::Sand);
                return true;
            }
        }

        return false;
    }
}

#[derive(Debug)]
struct RockPath {
    points: Vec<Point>,
}

impl FromStr for RockPath {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|point| point.parse().expect("Bad point!"))
            .collect();
        Ok(RockPath { points })
    }
}

impl IntoIterator for RockPath {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.points
            .windows(2)
            .map(|p| {
                let p1 = p.first().unwrap();
                let p2 = p.last().unwrap();
                Line::try_from((p1.clone(), p2.clone()))
                    .expect("Unable to create a line")
                    .into_iter()
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(anyhow!("Whoopsy!"))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
enum Line {
    Vertical((Point, Point)),
    Horizontal((Point, Point)),
}

impl TryFrom<(Point, Point)> for Line {
    type Error = Error;

    fn try_from((p0, p1): (Point, Point)) -> Result<Self, Self::Error> {
        match (p0.x.cmp(&p1.x), p0.y.cmp(&p1.y)) {
            (Ordering::Equal, Ordering::Less) => Ok(Line::Vertical((p0, p1))),
            (Ordering::Equal, Ordering::Greater) => Ok(Line::Vertical((p1, p0))),
            (Ordering::Less, Ordering::Equal) => Ok(Line::Horizontal((p0, p1))),
            (Ordering::Greater, Ordering::Equal) => Ok(Line::Horizontal((p1, p0))),
            (Ordering::Equal, Ordering::Equal) => Err(anyhow!("Points are the same!")),
            _ => Err(anyhow!("Diagonal line!")),
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Line::Vertical((p0, p1)) => (p0.y..=p1.y)
                .map(|y| Point { x: p0.x, y })
                .collect::<Vec<_>>()
                .into_iter(),
            Line::Horizontal((p0, p1)) => (p0.x..=p1.x)
                .map(|x| Point { x, y: p0.y })
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_into_iter_horizontal() {
        let p1 = Point::new(498, 4);
        let p2 = Point::new(498, 6);
        let mut iter = Line::try_from((p1, p2)).unwrap().into_iter();

        assert_eq!(iter.next().unwrap(), Point::new(498, 4));
        assert_eq!(iter.next().unwrap(), Point::new(498, 5));
        assert_eq!(iter.next().unwrap(), Point::new(498, 6));
        assert!(iter.next().is_none());
    }

    #[test]
    fn line_into_iter_vertical() {
        let p1 = Point::new(498, 6);
        let p2 = Point::new(496, 6);
        let mut iter = Line::try_from((p1, p2)).unwrap().into_iter();

        assert_eq!(iter.next().unwrap(), Point::new(496, 6));
        assert_eq!(iter.next().unwrap(), Point::new(497, 6));
        assert_eq!(iter.next().unwrap(), Point::new(498, 6));
        assert!(iter.next().is_none());
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 27551);
    }
}
