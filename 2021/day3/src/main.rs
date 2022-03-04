use std::fs;

fn mask_of_length(length: usize) -> u32 {
    let mut mask = 0;
    for i in 0..length {
        mask |= 1 << i;
    }

    mask
}

fn max_bit_count(arr: &[u32]) -> usize {
    let mut max = 0;

    for num in arr {
        let count = 32 - num.leading_zeros();
        if count > max {
            max = count;
        }
    }

    max as usize
}

fn main() {
    let content = fs::read_to_string("inputs/day3.txt").unwrap();
    let input: Vec<u32> = content.lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<u32>) -> u32 {
    let bit_count = max_bit_count(input);
    let mut frequencies = vec![0; bit_count];

    for &num in input {
        for i in 0..bit_count {
            let bit_pos = bit_count - 1 - i;
            let mask = 1 << bit_pos;
            frequencies[i] += if num & mask != 0 { 1 } else { -1 }
        }
    }

    let mut gamma: u32 = 0;

    for (i, &frequency) in frequencies.iter().enumerate() {
        if frequency > 0 {
            let bit_pos = bit_count - 1 - i;
            gamma |= 1 << bit_pos;
        }
    }

    let epsilon = !gamma & mask_of_length(bit_count);
    gamma * epsilon
}

fn get_rating(mut options: Vec<u32>, criteria: fn(bit: bool, frequency: i32) -> bool) -> u32 {
    let mut pos = max_bit_count(&options);

    while options.len() > 1 {
        pos -= 1;

        let mask = 1 << pos;
        let mut frequency = 0;

        for num in &options {
            frequency += if num & mask != 0 { 1 } else { -1 };
        }

        options.retain(|n| criteria(n & mask != 0, frequency));
    }

    match options.first() {
        Some(&num) => num,
        None => panic!("No oxygen generator rating found")
    }
}

fn part_2(input: &Vec<u32>) -> u32 {
    let oxygen_rating = get_rating(input.clone(), |bit, freq| (freq >= 0) == bit);
    let co2_rating = get_rating(input.clone(), |bit, freq| (freq < 0) == bit);
    oxygen_rating * co2_rating
}