use std::collections::HashSet;
use vector2d::Vector2D;
use aoc_lib::{AocSolution, BoxedError};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl From<&str> for Move {
    fn from(str: &str) -> Self {
        let (direction_char, distance_str) = str.split_once(' ').unwrap();

        Self {
            direction: match direction_char {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("invalid direction character"),
            },
            distance: distance_str.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Coordinates<T> {
    x: T,
    y: T,
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day9>(9)
}

fn simulate_rope(moves: &[Move], rope_len: usize) -> usize {
    let mut knots = vec![Vector2D::new(0, 0); rope_len];
    let mut visited_positions = HashSet::new();

    for Move { direction, distance } in moves {
        for _ in 0..*distance {
            match direction {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Right => knots[0].x += 1,
                Direction::Left => knots[0].x -= 1,
            }

            for i in 1..knots.len() {
                if knots[i - 1].y == knots[i].y {
                    let diff_x: i32 = knots[i - 1].x - knots[i].x;
                    if diff_x.abs() >= 2 {
                        knots[i].x += diff_x.signum();
                    }
                } else if knots[i - 1].x == knots[i].x {
                    let diff_y: i32 = knots[i - 1].y - knots[i].y;
                    if diff_y.abs() >= 2 {
                        knots[i].y += diff_y.signum();
                    }
                } else {
                    let diff = knots[i - 1] - knots[i];
                    if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                        knots[i].x += diff.x.signum();
                        knots[i].y += diff.y.signum();
                    }
                }
            }

            let last_knot = &knots.last().unwrap();
            visited_positions.insert((last_knot.x, last_knot.y));
        }
    }

    visited_positions.len()
}

struct Day9;

impl AocSolution for Day9 {
    type Input = Vec<Move>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(Move::from).collect()
    }

    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(simulate_rope(input, 2))
    }

    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(simulate_rope(input, 10))
    }
}
