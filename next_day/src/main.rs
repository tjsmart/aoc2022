use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::{cmp::Ordering, fs};

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;

fn main() -> Result<()> {
    let root_dir = get_root_dir()?;

    let src_dir = root_dir.join("src");
    if !src_dir.exists() {
        Err(anyhow!("Source directory does not exist: {:?}", src_dir))?;
    }

    let cargo_toml = root_dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        Err(anyhow!("Cargo.toml file not found!: {:?}", cargo_toml))?;
    }

    let session_file = root_dir.join(".session");
    if !session_file.exists() {
        Err(anyhow!(".session file not found!: {:?}", session_file))?;
    }

    let mut aoc_files = fs::read_dir(&src_dir)?
        .filter_map(|fname| match AocFile::try_from(fname.unwrap().path()) {
            Ok(aoc_file) => Some(aoc_file),
            Err(..) => None,
        })
        .collect::<Vec<_>>();
    aoc_files.sort();

    let prev_file = aoc_files.last().ok_or(anyhow!(
        "No files found in source directory: {:?}?!",
        src_dir
    ))?;
    let next_file = prev_file.next();

    fs::copy(
        src_dir.join(&prev_file.fname),
        src_dir.join(&next_file.fname),
    )
    .context(format!(
        "Failed to copy {} to {}",
        prev_file.fname, next_file.fname
    ))?;

    add_section(&cargo_toml, &next_file)?;
    if next_file.part == 1 {
        get_new_input(session_file, next_file.day)?;
    }

    Ok(())
}

fn get_new_input(session_file: PathBuf, day: u32) -> Result<()> {
    let session = fs::read_to_string(&session_file)?;
    let session = session.trim();

    let response = Client::new()
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("cookie", format!("session={}", session))
        .send()?;

    let new_input = session_file
        .parent()
        .unwrap()
        .join("input")
        .join(format!("day{:02}.txt", day));

    fs::write(new_input, response.text()?)?;

    Ok(())
}

fn add_section(cargo_toml: &PathBuf, next_file: &AocFile) -> Result<()> {
    let mut cargo_toml = OpenOptions::new().append(true).open(cargo_toml)?;
    write!(
        cargo_toml,
        "
[[bin]]
name = \"day{day:02}-{part}\"
path = \"src/{fname}\"
",
        day = next_file.day,
        part = next_file.part,
        fname = next_file.fname
    )?;
    Ok(())
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct AocFile {
    fname: String,
    day: u32,
    part: u32,
}

impl AocFile {
    fn new(day: u32, part: u32) -> Self {
        let fname = format!("day{day:02}-{part}.rs", day = day, part = part);
        AocFile { fname, day, part }
    }

    fn next(&self) -> Self {
        if self.part == 1 {
            AocFile::new(self.day, self.part + 1)
        } else {
            AocFile::new(self.day + 1, 1)
        }
    }
}

impl Ord for AocFile {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.day.cmp(&other.day) {
            Ordering::Equal => self.part.cmp(&other.part),
            ord => ord,
        }
    }
}

impl TryFrom<&str> for AocFile {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^day(\d{2})-(\d).rs$").unwrap();
        }

        let fname = s.to_string();
        let captures = RE
            .captures(&fname)
            .ok_or(anyhow!("file '{}', is not a match!", fname))?;

        Ok(AocFile {
            fname: fname.to_owned(),
            day: captures[1].parse()?,
            part: captures[2].parse()?,
        })
    }
}

impl TryFrom<PathBuf> for AocFile {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let s = path
            .file_name()
            .ok_or(anyhow!("Unable to get file_name from path"))?
            .to_str()
            .ok_or(anyhow!("Unable to convert path to a string"))?;
        AocFile::try_from(s)
    }
}

fn get_root_dir() -> Result<PathBuf> {
    let rootdir = String::from_utf8(
        Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()?
            .stdout,
    )?;

    Ok(PathBuf::from(rootdir.trim()))
}
