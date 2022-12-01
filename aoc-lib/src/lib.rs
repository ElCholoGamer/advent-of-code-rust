use dotenv::dotenv;
use std::{fs,env};
use std::path::PathBuf;
use chrono::{FixedOffset, TimeZone, Utc};
use reqwest::StatusCode;

pub mod utils;
mod error;

pub use error::Error;

fn get_base_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut dir = env::current_dir()?;
    if dir.file_name().unwrap().to_os_string().into_string().unwrap().starts_with("day") {
        dir.pop();
    }
    Ok(dir)
}

pub fn get_input<T, M: FnMut(&str) -> T>(day: u32, parse: M) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let base_dir = get_base_dir()?;
    let year: i32 = base_dir.file_name().unwrap().to_os_string().into_string().unwrap().parse()?;

    let input_dir = base_dir.join("cache");
    let input_path = input_dir.join(format!("{}.txt", day));

    if input_path.exists() {
        let contents = fs::read_to_string(&input_path)?;
        return Ok(contents.lines().map(parse).collect());
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
        .header("cookie", format!("session={}", sid))
        .send()?;

    if resp.status() == StatusCode::BAD_REQUEST {
        return Err(Error::InvalidSID.into());
    } else if resp.status() == StatusCode::NOT_FOUND {
        return Err(Error::InputUnavailable.into());
    }

    let input = resp.text()?;
    fs::create_dir_all(input_dir)?;
    fs::write(input_path, &input)?;

    Ok(input.lines().map(parse).collect())
}
