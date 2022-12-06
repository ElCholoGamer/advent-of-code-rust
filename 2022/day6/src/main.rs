use std::collections::HashMap;
use aoc_lib::{AocSolution, BoxedError, Error};

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day6>(6)
}

fn find_marker(message: &str, marker_len: usize) -> Option<usize> {
    let mut i = 0;
    'main: while i < message.len() - marker_len {
        let mut seen_char_indices = HashMap::new();

        for (c_index, char) in message[i..(i + marker_len)].chars().enumerate() {
            if let Some(previous_index) = seen_char_indices.insert(char, c_index) {
                i += previous_index + 1;
                continue 'main;
            }
        }

        return Some(i + marker_len);
    }

    None
}

struct Day6;

impl AocSolution for Day6 {
    type Input = String;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
    }

    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        find_marker(input, 4).ok_or(Error::Misc("could not find first marker".into()).into())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        find_marker(input, 14).ok_or(Error::Misc("could not find first marker".into()).into())
    }
}