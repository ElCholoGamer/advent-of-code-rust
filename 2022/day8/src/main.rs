use std::collections::{HashSet};
use aoc_lib::{AocSolution, BoxedError, Error};

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day8>(8)
}

enum Direction { Up, Down, Left, Right }

struct Day8;

impl AocSolution for Day8 {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        (0..lines[0].len()).map(|index| lines.iter().map(|line| line[index].to_digit(10).unwrap()).collect()).collect()
    }

    fn part_1(tree_heights: &Self::Input) -> Result<Self::Output, BoxedError> {
        let height = tree_heights[0].len();
        let width = tree_heights.len();
        let mut marked_visible = HashSet::new();

        for y in 0..height {
            let mut max_height = 0;

            for x in 0..width {
                if x == 0 || tree_heights[x][y] > max_height {
                    marked_visible.insert((x, y));
                    max_height = tree_heights[x][y];
                    if max_height == 9 { break; }
                }
            }

            max_height = 0;

            for x in (0..width).rev() {
                if x == width - 1 || tree_heights[x][y] > max_height {
                    marked_visible.insert((x, y));
                    max_height = tree_heights[x][y];
                    if max_height == 9 { break; }
                }
            }
        }

        for x in 0..width {
            let mut max_height = 0;

            for y in 0..height {
                if y == 0 || tree_heights[x][y] > max_height {
                    marked_visible.insert((x, y));
                    max_height = tree_heights[x][y];
                    if max_height == 9 { break; }
                }
            }

            max_height = 0;

            for y in (0..height).rev() {
                if y == height - 1 || tree_heights[x][y] > max_height {
                    marked_visible.insert((x, y));
                    max_height = tree_heights[x][y];
                    if max_height == 9 { break; }
                }
            }
        }

        Ok(marked_visible.len() as u32)
    }

    fn part_2(tree_heights: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut max_score = 0;

        for x in 0..tree_heights.len() {
            for y in 0..tree_heights[x].len() {
                let score = viewing_distance(tree_heights, x, y, Direction::Up)
                    * viewing_distance(tree_heights, x, y, Direction::Left)
                    * viewing_distance(tree_heights, x, y, Direction::Right)
                    * viewing_distance(tree_heights, x, y, Direction::Down);

                if score > max_score {
                    max_score = score;
                }
            }
        }

        Ok(max_score)
    }
}

fn viewing_distance(tree_heights: &Vec<Vec<u32>>, from_x: usize, from_y: usize, direction: Direction) -> u32 {
    let base_height = tree_heights[from_x][from_y];
    let mut viewing_distance = 0;

    match direction {
        Direction::Right => {
            for x in (from_x + 1)..tree_heights.len() {
                viewing_distance += 1;
                if tree_heights[x][from_y] >= base_height { break; }
            }
        }
        Direction::Left => {
            for x in (0..from_x).rev() {
                viewing_distance += 1;
                if tree_heights[x][from_y] >= base_height { break; }
            }
        }
        Direction::Down => {
            for y in (from_y + 1)..tree_heights[from_x].len() {
                viewing_distance += 1;
                if tree_heights[from_x][y] >= base_height { break; }
            }
        }
        Direction::Up => {
            for y in (0..from_y).rev() {
                viewing_distance += 1;
                if tree_heights[from_x][y] >= base_height { break; }
            }
        }
    }

    viewing_distance
}
