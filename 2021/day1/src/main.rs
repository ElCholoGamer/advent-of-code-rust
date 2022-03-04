use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let input: Vec<u32> = contents.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[u32]) -> u32 {
    let mut increases = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increases += 1;
        }
    }

    increases
}

fn part_2(input: &[u32]) -> u32 {
    let mut increments = 0;

    for i in 3..input.len() {
        let win1 = input[i - 1] + input[i - 2] + input[i - 3];
        let win2 = input[i] + input[i - 1] + input[i - 2];

        if win2 > win1 {
            increments += 1;
        }
    }

    increments
}
