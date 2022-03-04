use std::error::Error;
use std::fs;
use std::str::FromStr;

enum Action {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Action {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        let units = words[1].parse()?;

        match words[0] {
            "forward" => Ok(Action::Forward(units)),
            "down" => Ok(Action::Down(units)),
            "up" => Ok(Action::Up(units)),
            _ => Err("Invalid action command".into())
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let actions: Vec<Action> = input.lines().map(|l| l.parse().unwrap()).collect();

    println!("Part 1: {}", part_1(&actions));
    println!("Part 2: {}", part_2(&actions));
}

fn part_1(actions: &Vec<Action>) -> i32 {
    let mut pos = 0;
    let mut depth = 0;

    for action in actions {
        match action {
            Action::Forward(units) => pos += units,
            Action::Down(units) => depth += units,
            Action::Up(units) => depth -= units
        }
    }

    pos * depth
}

fn part_2(actions: &Vec<Action>) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;


    for action in actions {
        match action {
            Action::Forward(units) => {
                pos += units;
                depth += aim * units;
            }
            Action::Down(units) => aim += units,
            Action::Up(units) => aim -= units
        }
    }

    pos * depth
}
