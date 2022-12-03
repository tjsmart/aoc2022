use anyhow::Result;
use std::fmt::Debug;
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
