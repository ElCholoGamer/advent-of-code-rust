use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let input: Vec<u32> = contents.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[u32]) -> u32 {
    let mut sum = 0;

    for mass in input.iter() {
        sum += (mass / 3) - 2
    }

    sum
}

fn part_2(input: &[u32]) -> u32 {
    let mut sum = 0;

    for mass in input.iter() {
        let mut mass = *mass as i32;

        loop {
            mass = (mass / 3) - 2;
            if mass <= 0 { break; }

            sum += mass as u32;
        }
    }

    sum
}