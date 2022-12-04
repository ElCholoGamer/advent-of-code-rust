use std::ops::Range;
use aoc_lib::{AocSolution, BoxedError};

trait FromStrExt {
    fn from_str(s: &str) -> Self;
}

impl FromStrExt for Range<u32> {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day4>(4)
}

struct Day4;

impl AocSolution for Day4 {
    type Input = Vec<(Range<u32>, Range<u32>)>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(|line| {
            let (first, second) = s.split_once(',').unwrap();
            (Range::from_str(first), Range::from_str(second))
        }).collect()
    }

    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        let contained_pairs = input.iter().filter(|(first, second)|
            (first.start <= second.start && first.end >= second.end)
                || (second.start <= first.start && second.end >= first.end)
        );
        Ok(contained_pairs.count())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        let overlapping_pairs = input.iter().filter(|(first, second)|
            first.start <= second.end && first.end >= second.start
        );
        Ok(overlapping_pairs.count())
    }
}
