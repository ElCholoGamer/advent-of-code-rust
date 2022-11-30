use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let range: Vec<u32> = contents.split('-').map(|s| s.parse().unwrap()).collect();

    println!("Part 1: {}", part_1(range[0], range[1]));
    println!("Part 2: {}", part_2(range[0], range[1]));
}

fn part_1(min: u32, max: u32) -> u32 {
    let mut count = 0;

    for password in min..=max {
        let chars: Vec<char> = password.to_string().chars().collect();

        if has_adjacent_digits(&chars) && no_digits_decrease(&chars) {
            count += 1;
        }
    }

    count
}

fn part_2(min: u32, max: u32) -> u32 {
    let mut count = 0;

    for password in min..=max {
        let chars: Vec<char> = password.to_string().chars().collect();

        if has_unique_adjacent_digits(&chars) && no_digits_decrease(&chars) {
            count += 1;
        }
    }

    count
}

fn has_adjacent_digits(chars: &[char]) -> bool {
    let mut prev = '-';

    for &char in chars {
        if prev == char {
            return true;
        }

        prev = char;
    }

    false
}

fn no_digits_decrease(chars: &[char]) -> bool {
    for i in 1..chars.len() {
        if chars[i] < chars[i - 1] {
            return false;
        }
    }

    true
}

fn has_unique_adjacent_digits(chars: &[char]) -> bool {
    for i in 0..(chars.len() - 1) {
        let char = chars[i];
        if char == chars[i + 1] && char != chars[i - 1] {
            if i >= chars.len() - 2 || chars[i + 2] != char {
                return true;
            }
        }
    }

    false
}
