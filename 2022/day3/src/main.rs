use aoc_lib::{AocSolution, BoxedError};

fn item_priority(char: u8) -> u32 {
    if char >= b'a' {
        (char - b'a' + 1) as u32
    } else {
        (27 + char - b'A') as u32
    }
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day3>(3)
}

struct Day3;

impl AocSolution for Day3 {
    type Input = Vec<String>;
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(String::from).collect()
    }

    fn part_1(rucksacks: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(rucksacks.iter().fold(0, |sum, rucksack| {
            let bytes = rucksack.bytes().collect::<Vec<_>>();
            let mid = rucksack.len() / 2;
            let common_item = bytes[..mid].iter()
                .find(|byte| bytes[mid..].contains(byte))
                .expect("No common item type found");

            sum + item_priority(*common_item)
        }))
    }
    fn part_2(rucksacks: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(rucksacks.chunks_exact(3).fold(0, |sum, group| {
            let badge_item = &group[0].bytes()
                .find(|&byte| group[1].bytes().any(|b| b == byte) && group[2].bytes().any(|b| b == byte))
                .expect("No badge item type found");

            sum + item_priority(*badge_item)
        }))
    }
}
