use std::fmt::Display;
use std::str::FromStr;

use anyhow::Error;
use anyhow::Result;

use aoc::read_and_parse;
use aoc::time_it;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let grid = read_and_parse::<Grid>("input/day08.txt")?;
    let (ncols, nrows) = grid.shape();

    Ok((0..ncols)
        .map(|col| {
            (0..nrows)
                .filter_map(|row| grid.is_visible(col, row).then_some(1))
                .sum::<usize>()
        })
        .sum::<usize>())
}

impl Grid {
    fn is_visible(&self, col: usize, row: usize) -> bool {
        let (ncols, nrows) = self.shape();

        // from above?
        (0..col).all(|x| self.data[x][row] < self.data[col][row]) ||
        // from below?
        (col+1..ncols).rev().all(|x| self.data[x][row] < self.data[col][row]) ||
        // from left?
        (0..row).all(|x| self.data[col][x] < self.data[col][row]) ||
        // from right?
        (row+1..nrows).rev().all(|x| self.data[col][x] < self.data[col][row])
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<usize>>,
}

impl Grid {
    fn new(ncols: usize, nrows: usize) -> Self {
        Grid {
            data: vec![vec![0; nrows]; ncols],
        }
    }

    fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }

    fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &usize>> {
        self.data.iter().map(|row| row.iter())
    }

    fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &usize>> {
        let (ncols, nrows) = self.shape();
        (0..ncols).map(move |col| (0..nrows).map(move |row| &self.data[row][col]))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            let row = row.map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            data: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|x| x.to_digit(10).expect("why?") as usize)
                        .collect()
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_new() {
        let grid = Grid::new(3, 2);
        assert_eq!(grid.data[0][0], 0);
        assert_eq!(grid.data[1][0], 0);
        assert_eq!(grid.data[2][0], 0);
        assert_eq!(grid.data[0][1], 0);
        assert_eq!(grid.data[1][1], 0);
        assert_eq!(grid.data[2][1], 0);
    }

    #[test]
    fn grid_shape() {
        let grid = Grid::new(3, 2);
        assert_eq!(grid.shape(), (3, 2));
    }

    #[test]
    fn grid_from_str() {
        let grid = Grid::from_str("012\n345").unwrap();
        assert_eq!(grid.shape(), (2, 3));
    }

    #[test]
    fn grid_iter_rows() {
        let grid = Grid::from_str("01\n23").unwrap();
        let mut rows = grid.iter_rows();

        let mut row = rows.next().unwrap();
        assert_eq!(Some(&0), row.next());
        assert_eq!(Some(&1), row.next());
        assert_eq!(None, row.next());

        let mut row = rows.next().unwrap();
        assert_eq!(Some(&2), row.next());
        assert_eq!(Some(&3), row.next());
        assert_eq!(None, row.next());

        assert!(rows.next().is_none());
    }

    #[test]
    fn grid_iter_cols() {
        let grid = Grid::from_str("02\n13").unwrap();
        let mut cols = grid.iter_cols();

        let mut col = cols.next().unwrap();
        assert_eq!(Some(&0), col.next());
        assert_eq!(Some(&1), col.next());
        assert_eq!(None, col.next());

        let mut col = cols.next().unwrap();
        assert_eq!(Some(&2), col.next());
        assert_eq!(Some(&3), col.next());
        assert_eq!(None, col.next());

        assert!(cols.next().is_none());
    }

    // #[test]
    // fn sln() {
    //     assert_eq!(solution().unwrap(), 1117448);
    // }
}
