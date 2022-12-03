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
        let bytes = rucksack.bytes().collect::<Vec<_>>();
        let mid = rucksack.len() / 2;

        for &byte in bytes[..mid].iter() {
            if bytes[mid..].contains(&byte) {
                return sum + item_priority(byte);
            }
        }

        panic!("No common item type found");
    })
}

fn part_2(rucksacks: &[String]) -> u32 {
    rucksacks.chunks_exact(3).fold(0, |sum, group| {
        for byte in group[0].bytes() {
            if group[1].bytes().any(|b| b == byte) && group[2].bytes().any(|b| b == byte) {
                return sum + item_priority(byte);
            }
        }

        panic!("No badge item type found");
    })
}