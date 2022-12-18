use std::collections::HashSet;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use aoc::collect_lines;
use aoc::time_it;
use aoc::Point;

const ROW: i32 = 2000000;
type Vacancies = HashSet<i32>;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let readings = collect_lines::<Reading>("input/day15.txt")?;
    let mut vacancies = Vacancies::new();

    for reading in &readings {
        vacancies.extend(reading.vacancies());
    }

    for reading in &readings {
        if reading.beacon.y != ROW {
            continue;
        }
        vacancies.remove(&reading.beacon.x);
    }

    for reading in &readings {
        if reading.sensor.y != ROW {
            continue;
        }
        vacancies.remove(&reading.sensor.x);
    }

    Ok(vacancies.len())
}

#[derive(Debug)]
struct Reading {
    sensor: Point,
    beacon: Point,
}

impl Reading {
    ///
    ///```
    ///         dx
    ///   /-----------\
    ///   * * * * * * * * * * * *
    ///   * * * * * * * * * * * *
    ///   * * * * * * * * * * * *
    ///   * * * * * * S * * * * *  \
    ///   * * * * * * * * * * * *  │
    ///   * * * * * * * * * * * *  │ dy
    ///  -*-*-*-*-*-*-*-*-*-*-*-*- /    <- target row
    ///   * * * * B * * * * * * *
    ///   * * * * * * * * * * * *
    ///         \-----------/
    ///              num = 2 * (gamma) + 1
    ///
    ///  gamma = dx - dy
    ///```
    ///
    fn vacancies(&self) -> Vacancies {
        let dy = self.sensor.y.abs_diff(ROW) as i32;
        let b_rel = self.beacon - self.sensor;
        let dx = b_rel.x.abs() + b_rel.y.abs();
        let gamma = dx - dy;

        (-gamma..=gamma).map(|x| x + self.sensor.x).collect()
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

        Ok(Reading {
            sensor: (cap[1].parse()?, cap[2].parse()?).into(),
            beacon: (cap[3].parse()?, cap[4].parse()?).into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 5461729);
    }
}
