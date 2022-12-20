use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use aoc::collect_lines;
use aoc::time_it;
use aoc::Point;

const MAX: i32 = 4000000;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<u64> {
    let readings = collect_lines::<Reading>("input/day15.txt")?;

    let rslt = readings
        .iter()
        .map(|reading| reading.iter_border())
        .flatten()
        .filter(|pos| pos.x >= 0 && pos.x <= MAX && pos.y >= 0 && pos.y <= MAX)
        .find(|pos| {
            let rslt = !readings.iter().any(|reading| reading.is_vacant(pos));
            rslt
        })
        .ok_or(anyhow!("Oopsy!"))?;

    Ok((rslt.x as u64) * (4000000 as u64) + (rslt.y as u64))
}

#[derive(Debug)]
struct Reading {
    sensor: Point,
    beacon: Point,
    steps: i32,
}

impl Reading {
    fn new(sensor: Point, beacon: Point) -> Self {
        let b_rel = beacon - sensor;
        let steps = b_rel.x.abs() + b_rel.y.abs();
        Self {
            sensor,
            beacon,
            steps,
        }
    }

    fn iter_border(&self) -> impl Iterator<Item = Point> + '_ {
        let top_to_right = (0..=(self.steps + 1))
            .zip((0..=(self.steps + 1)).rev())
            .map(|dpos| self.sensor + dpos.into());

        let right_to_bottom = (0..=(-self.steps - 1))
            .zip((0..=(self.steps + 1)).rev())
            .map(|dpos| self.sensor + dpos.into());

        let top_to_left = (0..=(self.steps + 1))
            .zip((0..=(-self.steps - 1)).rev())
            .map(|dpos| self.sensor + dpos.into());

        let left_to_bottom = (0..=(-self.steps - 1))
            .zip((0..=(-self.steps - 1)).rev())
            .map(|dpos| self.sensor + dpos.into());

        top_to_right
            .chain(right_to_bottom)
            .chain(top_to_left)
            .chain(left_to_bottom)
    }

    fn is_vacant(&self, point: &Point) -> bool {
        let p_rel = *point - self.sensor;
        (p_rel.x.abs() + p_rel.y.abs()) <= self.steps
    }
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$"
            )
            .unwrap();
        }

        let cap = RE
            .captures(s)
            .ok_or(anyhow!("String does not match sensor regex: \n{}", s))?;

        Ok(Reading::new(
            (cap[1].parse()?, cap[2].parse()?).into(),
            (cap[3].parse()?, cap[4].parse()?).into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 10621647166538);
    }
}
