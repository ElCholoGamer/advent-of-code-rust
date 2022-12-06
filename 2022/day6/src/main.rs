use std::collections::HashSet;
use aoc_lib::{AocSolution, BoxedError, Error};

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day6>(6)
}

fn find_marker(message: &str, marker_len: usize) -> Option<usize> {
    message.chars().collect::<Vec<_>>()
        .windows(marker_len)
        .position(|chars| chars.iter().collect::<HashSet<_>>().len() == chars.len())
        .map(|sequence_index| sequence_index + marker_len)
}

struct Day6;

impl AocSolution for Day6 {
    type Input = String;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input { raw_input }

    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        find_marker(input, 4).ok_or(Error::Misc("could not find first marker".into()).into())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        find_marker(input, 14).ok_or(Error::Misc("could not find first marker".into()).into())
    }
}