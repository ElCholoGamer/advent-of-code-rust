use dotenv::dotenv;
use std::{fs, env};
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Instant;
use ansi_escapes::EraseLine;
use chrono::{FixedOffset, TimeZone, Utc};
use reqwest::StatusCode;
use colored::Colorize;

pub mod utils;
mod error;

pub use error::{Error, BoxedError};

pub trait AocSolution {
    type Input;
    type Output: Display;
    fn parse_input(raw_input: String) -> Self::Input;
    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError>;
    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError>;
}

fn get_input(day: u32, year: i32) -> Result<String, BoxedError> {
    let base_dir = get_base_dir()?;

    if env::args().any(|a| a == "--test" || a == "-T") {
        let test_input_path = base_dir.join("test").join(format!("{}.txt", day));
        if !test_input_path.exists() {
            return Err(Error::TestInputNotFound.into());
        }

        return Ok(fs::read_to_string(test_input_path)?);
    }

    let input_dir = base_dir.join("cache");
    let input_path = input_dir.join(format!("{}.txt", day));

    if input_path.exists() {
        return Ok(fs::read_to_string(&input_path)?);
    }

    let date = Utc::now() + FixedOffset::west_opt(5 * 3600).unwrap();
    let puzzle_date = Utc.with_ymd_and_hms(year, 12, day, 0, 0, 0).unwrap();
    if date < puzzle_date {
        return Err(Error::InputUnavailable.into());
    }

    dotenv().ok();
    let sid = env::var("SID_COOKIE").map_err(|_| Error::MissingSID)?;

    println!("Fetching puzzle input...");
    let resp = reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header("Cookie", format!("session={}", sid))
        .header("User-Agent", format!("github.com/ElCholoGamer/advent-of-code-rust v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS")))
        .send()?;

    if resp.status() == StatusCode::BAD_REQUEST {
        return Err(Error::InvalidSID.into());
    } else if resp.status() == StatusCode::NOT_FOUND {
        return Err(Error::InputUnavailable.into());
    }

    let contents = resp.text()?;
    fs::create_dir_all(input_dir)?;
    fs::write(input_path, &contents)?;

    Ok(contents)
}

pub fn run<S: AocSolution>(day: u32) -> Result<(), BoxedError> {
    let year: i32 = get_base_dir()?.file_name().unwrap().to_os_string().into_string().unwrap().parse()?;
    let raw_input = get_input(day, year)?;
    let input = S::parse_input(raw_input);

    println!("{}", format!("┌{:─^32}┐", "").blue());
    println!("{0} {1:^30} {0}", "│".blue(), format!("Advent of Code {} - Day {}", year, day).blue().bold());
    println!("{}", format!("└{:─^32}┘", "").blue());

    println!("{}", "Part 1".blue().bold());
    benchmark_part(|| S::part_1(&input));
    println!("{}", "Part 2".blue().bold());
    benchmark_part(|| S::part_2(&input));

    Ok(())
}

fn benchmark_part<F: Fn() -> Result<O, BoxedError>, O: Display>(f: F)  {
    let start = Instant::now();
    let output = f();
    let elapsed = start.elapsed();

    print!("\r{}", EraseLine);
    match output {
        Ok(result) => println!("{}", format!("Result: {}", result).green()),
        Err(e) => println!("{}", format!("{} {}", "Error:".bold(), e).red()),
    }
    println!("{}", format!("Elapsed: {}ms", elapsed.as_millis()).yellow());
}

fn get_base_dir() -> Result<PathBuf, BoxedError> {
    let mut dir = env::current_dir()?;
    if dir.file_name().unwrap().to_os_string().into_string().unwrap().starts_with("day") {
        dir.pop();
    }
    Ok(dir)
}
