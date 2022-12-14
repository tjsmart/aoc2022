use std::collections::HashSet;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::read_and_parse;
use aoc::time_it;

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
            current: HashSet::from([grid.end]),
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
        if self
            .current
            .iter()
            .any(|(py, px)| self.grid.elevations[*py][*px] == 0)
        {
            None
        } else {
            self.walk();
            Some(())
        }
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid {
    _start: Coord,
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
                pelev <= (nelev + 1)
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
            _start: start,
            end,
            elevations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 465);
    }
}
