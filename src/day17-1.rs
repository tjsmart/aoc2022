use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::iter::Cycle;
use std::ops::Add;
use std::vec::IntoIter;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::time_it;
use aoc::Point;
use itertools::Itertools;

const NUM_ROCKS: usize = 2022;
const XSTART: i32 = 2;
const YSTART_OFFSET: i32 = 3;
const FLOOR_SIZE: usize = 7;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

/*
 * |....#..|
 * |....#..|
 * |....##.|
 * |##..##.|
 * |######.|
 * |.###...|
 * |..#....|
 * |.####..|
 * |....##.|
 * |....##.|
 * |....#..|
 * |..#.#..|
 * |..#.#..|
 * |#####..|
 * |..###..|
 * |...#...|
 * |..####.|
 */

fn solution() -> Result<i32> {
    let shifts = read_shifts("input/day17.txt")?;
    let mut game = GameIter::new(FLOOR_SIZE, shifts);

    Ok(game.nth(NUM_ROCKS - 1).unwrap())
}

struct GameIter {
    game: Game,
    shifts: Cycle<IntoIter<Shift>>,
    cycle: usize,
}

impl GameIter {
    fn new(width: usize, shifts: Vec<Shift>) -> Self {
        Self {
            game: Game::new(width),
            shifts: shifts.into_iter().cycle(),
            cycle: 0,
        }
    }
}

impl Iterator for GameIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;
        loop {
            self.game
                .step(self.shifts.next().expect("Ran out of shifts!"));
            if self.game.rock.is_none() {
                break;
            }
        }

        // print!("{esc}c", esc = 27 as char);
        // println!("{}", self.game);
        // println!("{}", self.cycle);
        // std::io::stdin()
        //     .read_line(&mut String::new())
        //     .expect("failed to read input");

        Some(self.game.height)
    }
}

#[derive(Debug)]
struct Game {
    width: i32,
    rock: Option<Rock>,
    rocks_spawn: Cycle<RockIter>,
    ground: HashSet<Point>,
    height: i32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rock_ymax = self
            .rock
            .as_ref()
            .and_then(|rock| Some(rock.ymax()))
            .unwrap_or(0);

        for iy in (0..=self.height.max(rock_ymax)).rev() {
            write!(f, "+")?;
            let line = (0..self.width)
                .map(|ix| {
                    if self.ground.contains(&(ix, iy).into()) {
                        '#'
                    } else if self
                        .rock
                        .as_ref()
                        .and_then(|rock| Some(rock.pixels().contains(&(ix, iy).into())))
                        .unwrap_or(false)
                    {
                        '@'
                    } else {
                        '.'
                    }
                })
                .join("");
            writeln!(f, "{}+", line)?;
        }

        writeln!(f, "+-------+")?;
        Ok(())
    }
}

impl Game {
    fn new(width: usize) -> Self {
        Self {
            width: width as i32,
            rock: None,
            rocks_spawn: RockIter::default().cycle(),
            ground: HashSet::new(),
            height: 0,
        }
    }

    fn spawn_rock(&mut self) {
        let mut new_rock = self.rocks_spawn.next().unwrap();
        let ystart = self.height + YSTART_OFFSET;
        new_rock.shift((XSTART, ystart).into());
        self.rock = Some(new_rock);
    }

    fn step(&mut self, shift: Shift) {
        if self.rock.is_none() {
            self.spawn_rock();
        }

        // shift left/right
        self.try_shift(shift.into());

        // try to drop
        if !self.try_shift(Point::new(0, -1)) {
            let rested_rock = self.rock.take().unwrap();
            self.height = (rested_rock.ymax() + 1).max(self.height);
            self.ground.extend(rested_rock.pixels());
        }
    }

    fn try_shift(&mut self, shift: Point) -> bool {
        let rock = self.rock.as_mut().unwrap();

        let can_move = rock.pixels().map(|pixel| pixel + shift).all(|pixel| {
            !self.ground.contains(&pixel) && pixel.x >= 0 && pixel.x < self.width && pixel.y >= 0
        });

        // println!("{rock:?} shift {shift:?}?, {can_move:?}");

        if can_move {
            rock.shift(shift);
        }

        can_move
    }
}

fn read_shifts(fname: &str) -> Result<Vec<Shift>> {
    std::fs::read_to_string(fname)?
        .trim()
        .chars()
        .map(|char| char.try_into())
        .collect::<Result<_>>()
}

#[derive(Debug, Clone)]
enum Shift {
    Left,
    Right,
}

impl Display for Shift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.into();
        write!(f, "{}", c)
    }
}

impl From<&Shift> for char {
    fn from(shift: &Shift) -> char {
        match shift {
            Shift::Left => '<',
            Shift::Right => '>',
        }
    }
}

impl TryFrom<char> for Shift {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(anyhow!("Unrecognized shift character: '{}'", c)),
        }
    }
}

impl Into<Point> for Shift {
    fn into(self) -> Point {
        match self {
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0),
        }
    }
}

type Pixels = HashMap<i32, Vec<i32>>;

#[derive(Debug)]
struct Rock {
    _pixels: Pixels,
    _offset: Point,
}

impl Rock {
    fn empty() -> Self {
        Self::new(Pixels::new())
    }

    fn new(pixels: Pixels) -> Self {
        Self {
            _pixels: pixels,
            _offset: Point::default(),
        }
    }

    fn shift(&mut self, dpos: Point) {
        self._offset += dpos;
    }

    fn pixels(&self) -> impl Iterator<Item = Point> + '_ {
        self._pixels
            .iter()
            .map(|(x, ys)| ys.iter().map(|y| Point::new(*x, *y) + self._offset))
            .flatten()
    }

    fn bottom(&self) -> impl Iterator<Item = Point> + '_ {
        self._pixels
            .iter()
            .filter_map(|(x, ys)| ys.iter().min().map(|y| Point::new(*x, *y) + self._offset))
    }

    fn top(&self) -> impl Iterator<Item = Point> + '_ {
        self._pixels
            .iter()
            .filter_map(|(x, ys)| ys.iter().max().map(|y| Point::new(*x, *y) + self._offset))
    }

    fn xmin(&self) -> i32 {
        self.pixels().map(|pixel| pixel.x).min().unwrap()
    }

    fn xmax(&self) -> i32 {
        self.pixels().map(|pixel| pixel.x).max().unwrap()
    }

    fn ymin(&self) -> i32 {
        self.pixels().map(|pixel| pixel.y).min().unwrap()
    }

    fn ymax(&self) -> i32 {
        self.pixels().map(|pixel| pixel.y).max().unwrap()
    }
}

impl Add<Point> for Rock {
    type Output = Rock;
    fn add(self, rhs: Point) -> Self::Output {
        Self {
            _pixels: self._pixels,
            _offset: self._offset + rhs,
        }
    }
}

impl FromIterator<Point> for Rock {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut new = Self::empty();
        for pixel in iter {
            if let Some(ys) = new._pixels.get_mut(&pixel.x) {
                ys.push(pixel.y);
            } else {
                new._pixels.insert(pixel.x, Vec::from([pixel.y]));
            }
        }

        new
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ymax = self.pixels().map(|point| point.y).max().unwrap_or(0);
        let xmax = self.pixels().map(|point| point.x).max().unwrap_or(0);

        for iy in 0..=ymax {
            let line = (0..=xmax)
                .map(|ix| {
                    if self.pixels().contains(&(ix, iy).into()) {
                        "#"
                    } else {
                        "."
                    }
                })
                .join("");

            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RockIter {
    count: usize,
}

impl RockIter {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for RockIter {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.count {
            // ####
            0 => Some(Rock::from_iter(
                [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(2, 0),
                    Point::new(3, 0),
                ]
                .into_iter(),
            )),

            // .#.
            // ###
            // .#.
            1 => Some(Rock::from_iter(
                [
                    Point::new(1, 0),
                    Point::new(0, 1),
                    Point::new(1, 1),
                    Point::new(2, 1),
                    Point::new(1, 2),
                ]
                .into_iter(),
            )),

            // ..#
            // ..#
            // ###
            2 => Some(Rock::from_iter(
                [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(2, 0),
                    Point::new(2, 1),
                    Point::new(2, 2),
                ]
                .into_iter(),
            )),

            // #
            // #
            // #
            // #
            3 => Some(Rock::from_iter(
                [
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(0, 3),
                ]
                .into_iter(),
            )),

            // ##
            // ##
            4 => Some(Rock::from_iter(
                [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(0, 1),
                    Point::new(1, 1),
                ]
                .into_iter(),
            )),

            _ => None,
        };

        self.count += 1;
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 3111);
    }
}
