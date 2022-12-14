use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;
// use std::thread;
// use std::time;
// use std::time::Duration;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::read_and_parse;
use aoc::time_it;
use itertools::Itertools;

// const SLEEP: Duration = time::Duration::from_millis(30);

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let grid = read_and_parse::<Grid>("input/day12.txt")?;
    let visitor = Visitor::new(grid);
    Ok(visitor.count())
}

#[derive(Debug)]
struct Visitor {
    current: HashSet<Coord>,
    prev: HashSet<Coord>,
    grid: Grid,
}

impl Visitor {
    fn new(grid: Grid) -> Self {
        Self {
            current: HashSet::from([grid.start]),
            prev: HashSet::new(),
            grid,
        }
    }

    fn walk(&mut self) {
        self.prev.extend(self.current.iter());
        self.current = self
            .current
            .iter()
            .map(|pos| {
                let neighbors = self.grid.accessible_neighbors(*pos);
                neighbors
                    .into_iter()
                    .filter(|neighbor| !self.prev.contains(neighbor))
            })
            .flatten()
            .collect();
    }
}

impl Iterator for Visitor {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        // print!("{esc}c", esc = 27 as char);
        // println!("{self}");
        // thread::sleep(SLEEP);
        if self.current.contains(&self.grid.end) {
            None
        } else {
            self.walk();
            Some(())
        }
    }
}

impl Display for Visitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (nrows, ncols) = self.grid.shape();

        for row in 0..nrows {
            let line = (0..ncols).map(|col| _to_char(self, row, col)).join("");
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn _to_char(visitor: &Visitor, row: usize, col: usize) -> char {
    let start = visitor.grid.start;
    let end = visitor.grid.end;

    if (row, col) == start {
        'S'
    } else if (row, col) == end {
        'E'
    } else {
        if visitor.prev.contains(&(row, col)) {
            '.'
        } else {
            'x'
        }
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid {
    start: Coord,
    end: Coord,
    elevations: Vec<Vec<u8>>,
}

impl Grid {
    fn shape(&self) -> (usize, usize) {
        (self.elevations.len(), self.elevations[0].len())
    }

    fn neighbors(&self, pos: Coord) -> HashSet<Coord> {
        let (nrows, ncols) = self.shape();

        vec![
            checked_decrement(pos.0).and_then(|y| Some((y, pos.1))),
            checked_increment(pos.0, nrows).and_then(|y| Some((y, pos.1))),
            checked_decrement(pos.1).and_then(|x| Some((pos.0, x))),
            checked_increment(pos.1, ncols).and_then(|x| Some((pos.0, x))),
        ]
        .into_iter()
        .filter_map(|x| x)
        .collect()
    }

    fn accessible_neighbors(&self, pos: Coord) -> HashSet<Coord> {
        let (py, px) = pos;
        self.neighbors(pos)
            .into_iter()
            .filter(|(ny, nx)| {
                let pelev = self.elevations[py][px];
                let nelev = self.elevations[*ny][*nx];
                nelev <= (pelev + 1)
            })
            .collect()
    }
}

fn checked_increment(x: usize, limit: usize) -> Option<usize> {
    (x < limit - 1).then(|| x + 1)
}

fn checked_decrement(x: usize) -> Option<usize> {
    (x > 0).then(|| x - 1)
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elevations = s
            .lines()
            .map(|line| Vec::from_iter(line.chars()))
            .collect::<Vec<_>>();
        let start = elevations
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, col)| (*col == 'S').then_some((i, j)))
            })
            .ok_or(anyhow!("failed to find start!"))?;
        let end = elevations
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, col)| (*col == 'E').then_some((i, j)))
            })
            .ok_or(anyhow!("failed to find end!"))?;
        let elevations = elevations
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|digit| match digit {
                        'S' => 0,
                        'E' => ('z' as u8) - ('a' as u8),
                        x => (x as u8) - ('a' as u8),
                    })
                    .collect()
            })
            .collect();

        Ok(Grid {
            start,
            end,
            elevations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    const INPUT: &str = "\
abcd
SabE
zyxw
";

    const NROWS: usize = 3;
    const NCOLS: usize = 4;

    #[test]
    fn checked_decrement_test() {
        assert_eq!(checked_decrement(0), None);
        assert_eq!(checked_decrement(1), Some(0));
        assert_eq!(checked_decrement(10), Some(9));
    }

    #[test]
    fn checked_increment_test() {
        assert_eq!(checked_increment(8, 10), Some(9));
        assert_eq!(checked_increment(9, 10), None);
        assert_eq!(checked_increment(10, 10), None);
    }

    #[test]
    fn grid_from_str() {
        let expected = vec![vec![0, 1, 2, 3], vec![0, 0, 1, 25], vec![25, 24, 23, 22]];
        let grid = Grid::from_str(INPUT).unwrap();

        assert_eq!(grid.elevations, expected);
    }

    #[test]
    fn grid_shape() {
        let grid = Grid::from_str(INPUT).unwrap();
        assert_eq!(grid.shape(), (NROWS, NCOLS));
    }

    #[rstest]
    // corners
    #[case((0, 0), HashSet::from([(1, 0), (0, 1)]))]
    #[case((NROWS-1, 0), HashSet::from([(NROWS-2, 0), (NROWS-1, 1)]))]
    #[case((0, NCOLS-1), HashSet::from([(0, NCOLS-2), (1, NCOLS-1)]))]
    #[case((NROWS-1, NCOLS-1), HashSet::from([(NROWS-1, NCOLS-2), (NROWS-2, NCOLS-1)]))]
    // edges
    #[case((1, 0), HashSet::from([(0, 0), (2, 0), (1, 1)]))]
    #[case((0, 1), HashSet::from([(1, 1), (0, 0), (0, 2)]))]
    #[case((NROWS-2, NCOLS-1), HashSet::from([(NROWS-1, NCOLS-1), (NROWS-3, NCOLS-1), (NROWS-2, NCOLS-2)]))]
    #[case((NROWS-1, NCOLS-2), HashSet::from([(NROWS-1, NCOLS-1), (NROWS-1, NCOLS-3), (NROWS-2, NCOLS-2)]))]
    // middle
    #[case((1, 1), HashSet::from([(0, 1), (1, 0), (2, 1), (1, 2)]))]
    fn grid_neighbors(#[case] pos: Coord, #[case] expected: HashSet<Coord>) {
        let grid = Grid::from_str(INPUT).unwrap();

        assert_eq!(grid.neighbors(pos), expected);
    }

    #[rstest]
    // corners
    #[case((0, 0), HashSet::from([(1, 0), (0, 1)]))]
    #[case((NROWS-1, 0), HashSet::from([(NROWS-2, 0), (NROWS-1, 1)]))]
    #[case((0, NCOLS-1), HashSet::from([(0, NCOLS-2)]))]
    #[case((NROWS-1, NCOLS-1), HashSet::from([(NROWS-1, NCOLS-2)]))]
    // edges
    #[case((1, 0), HashSet::from([(0, 0), (1, 1)]))]
    #[case((0, 1), HashSet::from([(1, 1), (0, 0), (0, 2)]))]
    #[case((NROWS-2, NCOLS-1), HashSet::from([(NROWS-1, NCOLS-1), (NROWS-3, NCOLS-1), (NROWS-2, NCOLS-2)]))]
    #[case((NROWS-1, NCOLS-2), HashSet::from([(NROWS-1, NCOLS-1), (NROWS-1, NCOLS-3), (NROWS-2, NCOLS-2)]))]
    // middle
    #[case((1, 1), HashSet::from([(0, 1), (1, 0), (1, 2)]))]
    fn grid_accessible_neighbors(#[case] pos: Coord, #[case] expected: HashSet<Coord>) {
        let grid = Grid::from_str(INPUT).unwrap();

        assert_eq!(grid.accessible_neighbors(pos), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 472);
    }
}
