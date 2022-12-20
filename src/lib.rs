use anyhow::Result;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::time;

pub fn collect_lines<T>(fname: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    Ok(std::fs::read_to_string(&fname)?
        .lines()
        .map(|line| line.parse::<T>().expect("Failed to parse line."))
        .collect())
}

pub fn collect_blocks<T>(fname: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    Ok(std::fs::read_to_string(&fname)?
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.parse::<T>().expect("Failed to parse line."))
                .collect()
        })
        .collect())
}

pub fn collect_statements<T>(fname: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    Ok(std::fs::read_to_string(&fname)?
        .split("\n\n")
        .map(|block| block.parse().expect("Failed to parse statement."))
        .collect())
}

pub fn read_and_parse<T>(fname: &str) -> Result<T>
where
    T: FromStr<Err = anyhow::Error>,
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(&fname)?.parse()
}

pub fn time_it<F, R>(func: F) -> Result<()>
where
    F: FnOnce() -> Result<R>,
    R: std::fmt::Debug + std::fmt::Display,
{
    let now = time::Instant::now();
    let answer = func()?;
    let duration = now.elapsed().as_micros();
    println!("{answer}");
    println!("{duration} us");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Point {
    pub fn default() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
