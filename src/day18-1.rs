use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Mul;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;

use aoc::collect_lines;
use aoc::time_it;

type Faces = [Point; 6];

const DEFAULT_FACES: Faces = [
    Point::new(0, 1, 1),
    Point::new(2, 1, 1),
    Point::new(1, 0, 1),
    Point::new(1, 2, 1),
    Point::new(1, 1, 0),
    Point::new(1, 1, 2),
];

fn main() -> Result<()> {
    time_it(solution)
}

fn solution() -> Result<usize> {
    let droplets = collect_lines::<Droplet>("input/day18.txt")?;
    let face_counter = droplets
        .into_iter()
        .map(|droplet| droplet.faces())
        .flatten()
        .counter();

    Ok(face_counter
        .counts
        .into_iter()
        .filter(|(_, v)| *v == 1)
        .count())
}

struct Counts<K> {
    counts: HashMap<K, usize>,
}

impl<K> Counts<K> {
    fn new(iter: impl Iterator<Item = K>) -> Counts<K>
    where
        K: Hash + Eq,
    {
        let mut counts: HashMap<K, usize> = HashMap::new();
        for item in iter {
            let count = counts.get(&item).unwrap_or(&0);
            counts.insert(item, *count + 1);
        }
        Counts { counts }
    }
}

trait Counter<K> {
    fn counter(self) -> Counts<K>;
}

impl<I, K> Counter<K> for I
where
    I: Iterator<Item = K>,
    K: Hash + Eq,
{
    fn counter(self) -> Counts<K> {
        Counts::new(self)
    }
}

// pub struct Enumerate<I> {
//     iter: I,
//     count: usize,
// }
// impl<I> Enumerate<I> {
//     pub(in crate::iter) fn new(iter: I) -> Enumerate<I> {
//         Enumerate { iter, count: 0 }
//     }
// }
//
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<I> Iterator for Enumerate<I>
// where
//     I: Iterator,
// {
//     type Item = (usize, <I as Iterator>::Item);
//
//     #[inline]
//     #[rustc_inherit_overflow_checks]
//     fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
//         let a = self.iter.next()?;
//         let i = self.count;
//         self.count += 1;
//         Some((i, a))
//     }

#[derive(Debug)]
struct Droplet {
    offset: Point,
}

impl FromStr for Droplet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s.split(',').collect_tuple() {
            Ok(Self {
                offset: Point::new(x.parse()?, y.parse()?, z.parse()?),
            })
        } else {
            Err(anyhow!("Invalid droplet string: {}", s))
        }
    }
}

impl Droplet {
    fn faces(&self) -> Vec<Point> {
        DEFAULT_FACES
            .iter()
            .map(|&face| face + (self.offset * 2))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("0,0,0", vec![
            Point::new(0, 1, 1),
            Point::new(2, 1, 1),
            Point::new(1, 0, 1),
            Point::new(1, 2, 1),
            Point::new(1, 1, 0),
            Point::new(1, 1, 2),
        ]
    )]
    #[case("1,0,0", vec![
            Point::new(2, 1, 1),
            Point::new(4, 1, 1),
            Point::new(3, 0, 1),
            Point::new(3, 2, 1),
            Point::new(3, 1, 0),
            Point::new(3, 1, 2),
        ]
    )]
    fn get_array_indices_test(#[case] input: &str, #[case] expected: Vec<Point>) {
        assert_eq!(input.parse::<Droplet>().unwrap().faces(), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 4320);
    }
}
