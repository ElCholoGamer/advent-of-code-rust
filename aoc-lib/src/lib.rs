use dotenv::dotenv;
use std::fs;
use std::path::Path;
use chrono::{FixedOffset, TimeZone, Utc};

pub mod utils;
mod error;

pub use error::Error;

pub fn get_input<T, M: FnMut(&str) -> T>(year: i32, day: u32, parse: M) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file_dir = Path::new("cache").join(year.to_string());
    let file_path = file_dir.join(format!("{}.txt", day));

    if file_path.exists() {
        let contents = fs::read_to_string(&file_path)?;
        return Ok(contents.lines().map(parse).collect());
    }

    let date = Utc::now() + FixedOffset::west_opt(5 * 3600).unwrap();
    let puzzle_date = Utc.with_ymd_and_hms(year, 12, day, 0, 0, 0).unwrap();
    if date < puzzle_date {
        return Err(Error::InputUnavailable.into());
    }

    dotenv().ok();
    let sid = std::env::var("SID_COOKIE").map_err(|_| Error::MissingSID)?;

    println!("Fetching puzzle input...");
    let resp = reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header("cookie", format!("session={}", sid))
        .send()?;

    if resp.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err(Error::InvalidSID.into());
    }

    let input = resp.text()?;
    fs::create_dir_all(file_dir)?;
    fs::write(file_path, &input)?;

    Ok(input.lines().map(parse).collect())
}
