use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
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
    // let mut droplets = collect_lines::<Droplet>("example.txt")?;
    let mut droplets = collect_lines::<Droplet>("input/day18.txt")?;
    let air_pockets = find_air_pockets(&droplets);
    for air_pocket in air_pockets {
        droplets.push(air_pocket.into());
    }

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

fn find_air_pockets(droplets: &[Droplet]) -> Vec<Point> {
    let unavailable_points = droplets
        .iter()
        .map(|droplet| droplet.offset)
        .collect::<HashSet<_>>();

    let span = calc_span(droplets);

    let mut candidates: HashSet<_> = ((span.0.x + 1)..span.1.x)
        .map(|x| {
            ((span.0.y + 1)..span.1.y)
                .map(move |y| ((span.0.z + 1)..span.1.z).map(move |z| Point::new(x, y, z)))
                .flatten()
        })
        .flatten()
        .filter(|point| !unavailable_points.contains(&point))
        .collect();

    let mut air_pockets = Vec::new();

    while let Some(candidate) = candidates.pop() {
        if let Some(pocket) = check_pocket(candidate, &mut candidates, &unavailable_points) {
            air_pockets.extend(pocket.points);
        }
    }

    air_pockets
}

fn check_pocket(
    candidate: Point,
    candidates: &mut HashSet<Point>,
    unavailable_points: &HashSet<Point>,
) -> Option<Pocket> {
    let mut pocket = Pocket::from(candidate);

    while let Some(candidate) = pocket.peak() {
        let candidate_at_edge = empty_neighbors(candidate, &unavailable_points).any(|neighbor| {
            if pocket.contains(&neighbor) {
                false
            } else if candidates.remove(&neighbor) {
                pocket.push(neighbor);
                false
            } else {
                true
            }
        });

        if candidate_at_edge {
            return None;
        }
    }

    Some(pocket)
}

trait Pop<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<T> Pop<T> for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.take(&self.iter().next().unwrap().clone())
        }
    }
}

fn empty_neighbors(
    candidate: Point,
    unavailable_points: &HashSet<Point>,
) -> impl Iterator<Item = Point> + '_ {
    neighbors(candidate).filter(move |candidate| !unavailable_points.contains(&candidate))
}

fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    [
        Point::new(1, 0, 0),
        Point::new(0, 1, 0),
        Point::new(0, 0, 1),
    ]
    .into_iter()
    .map(move |dir| [point + dir, point - dir].into_iter())
    .flatten()
}

struct Pocket {
    points: Vec<Point>,
    idx: usize,
}

impl Pocket {
    fn push(&mut self, point: Point) {
        self.points.push(point);
    }

    fn peak(&mut self) -> Option<Point> {
        if let Some(next) = self.points.get(self.idx) {
            self.idx += 1;
            Some(*next)
        } else {
            None
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }
}

impl From<Point> for Pocket {
    fn from(point: Point) -> Self {
        Self {
            points: vec![point],
            idx: 0,
        }
    }
}

fn calc_span(droplets: &[Droplet]) -> (Point, Point) {
    (
        Point::new(
            droplets
                .iter()
                .map(|droplet| droplet.offset.x)
                .min()
                .unwrap_or(0),
            droplets
                .iter()
                .map(|droplet| droplet.offset.y)
                .min()
                .unwrap_or(0),
            droplets
                .iter()
                .map(|droplet| droplet.offset.z)
                .min()
                .unwrap_or(0),
        ),
        Point::new(
            droplets
                .iter()
                .map(|droplet| droplet.offset.x)
                .max()
                .unwrap_or(0),
            droplets
                .iter()
                .map(|droplet| droplet.offset.y)
                .max()
                .unwrap_or(0),
            droplets
                .iter()
                .map(|droplet| droplet.offset.z)
                .max()
                .unwrap_or(0),
        ),
    )
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

impl From<Point> for Droplet {
    fn from(offset: Point) -> Self {
        Self { offset }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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
        assert_eq!(solution().unwrap(), 2456);
    }
}
