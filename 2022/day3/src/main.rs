use std::collections::HashSet;
use aoc_lib::BoxedError;

fn item_priority(char: u8) -> u32 {
    if char >= b'a' {
        (char - b'a' + 1) as u32
    } else {
        (27 + char - b'A') as u32
    }
}

fn main() -> Result<(), BoxedError> {
    let rucksacks = aoc_lib::get_input(3, |l| Ok(String::from(l)))?;
    println!("Part 1: {}", part_1(&rucksacks));
    println!("Part 2: {}", part_2(&rucksacks));
    Ok(())
}

fn part_1(rucksacks: &[String]) -> u32 {
    rucksacks.iter().fold(0, |sum, rucksack| {
        let mut s = 0u32;

        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        let unique_bytes_1 = first.bytes().collect::<HashSet<_>>();
        let unique_bytes_2 = second.bytes().collect::<HashSet<_>>();
        for byte in unique_bytes_1 {
            if unique_bytes_2.contains(&byte) {
                s += item_priority(byte);
            }
        }

        sum + s
    })
}

fn part_2(rucksacks: &[String]) -> u32 {
    let groups = rucksacks.chunks_exact(3).collect::<Vec<_>>();

    groups.iter().fold(0, |sum, &group| {
        let unique_bytes_1 = group[0].bytes().collect::<HashSet<_>>();
        let unique_bytes_2 = group[1].bytes().collect::<HashSet<_>>();
        let unique_bytes_3 = group[2].bytes().collect::<HashSet<_>>();

        for byte in unique_bytes_1 {
            if unique_bytes_2.contains(&byte) && unique_bytes_3.contains(&byte) {
                return sum + item_priority(byte);
            }
        }

        panic!("No badge item type found");
    })
}