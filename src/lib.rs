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
