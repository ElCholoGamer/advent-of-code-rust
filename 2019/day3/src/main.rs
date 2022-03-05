use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use vector2d::Vector2D;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    distance: i32,
    direction: Direction,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s[1..].parse::<i32>().map_err(|e| e.to_string())?;

        let direction = match s.chars().next().unwrap() {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => return Err("Invalid direction".into()),
        };

        Ok(Self { distance, direction })
    }
}

fn parse_moves(str: &str) -> Result<Vec<Move>, String> {
    str.clone().split(',').map(|m| m.parse::<Move>()).collect()
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let wire1 = parse_moves(lines[0]).unwrap();
    let wire2 = parse_moves(lines[1]).unwrap();
    let wires = [wire1, wire2];

    println!("Part 1: {}", part_1(&wires).unwrap());
    println!("Part 2: {}", part_2(&wires).unwrap());
}

fn part_1(wires: &[Vec<Move>]) -> Result<i32, &str> {
    let intersections = find_intersections(&wires);
    let closest_distance = intersections.iter()
        .map(|(x, y)| x.abs() + y.abs())
        .reduce(|a, b| if a < b { a } else { b });

    match closest_distance {
        None => Err("No intersections found"),
        Some(d) => Ok(d)
    }
}

fn part_2(wires: &[Vec<Move>]) -> Result<i32, &str> {
    let intersections = find_intersections(&wires);
    let total_distances: Vec<i32> = intersections.iter().map(|intersection| {
        let mut distance = 0;

        for moves in wires {
            let mut pos = Vector2D::<i32>::new(0, 0);
            let mut steps = 0;

            'main: for wire_move in moves {
                for _ in 0..wire_move.distance {
                    step_position(&mut pos, &wire_move.direction);
                    steps += 1;

                    if (pos.x, pos.y) == *intersection {
                        break 'main;
                    }
                }
            }

            distance += steps;
        }

        distance
    }).collect();

    let closest_distance = total_distances.iter().reduce(|a, b| if a < b { a } else { b });

    match closest_distance {
        None => Err("No intersections found"),
        Some(d) => Ok(*d),
    }
}

fn step_position(position: &mut Vector2D<i32>, direction: &Direction) {
    match direction {
        Direction::Up => position.y -= 1,
        Direction::Down => position.y += 1,
        Direction::Right => position.x += 1,
        Direction::Left => position.x -= 1,
    }
}

fn find_intersections(wires: &[Vec<Move>]) -> Vec<(i32, i32)> {
    let mut grid = HashMap::<(i32, i32), u32>::new();

    for moves in wires {
        let mut pos = Vector2D::new(0, 0);
        let mut visited = HashSet::<(i32, i32)>::new();

        for wire_move in moves {
            for _ in 0..wire_move.distance {
                step_position(&mut pos, &wire_move.direction);
                visited.insert((pos.x, pos.y));
            }
        }

        for coord in visited {
            grid.insert(coord, grid.get(&coord).unwrap_or(&0) + 1);
        }
    }

    grid.keys().into_iter().filter(|k| grid[k] >= 2).map(|c| c.clone()).collect()
}