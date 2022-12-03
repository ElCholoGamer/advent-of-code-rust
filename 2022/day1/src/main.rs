use aoc_lib::{AocSolution, BoxedError};

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day1>(1)
}

struct Day1;

impl AocSolution for Day1 {
    type Input = Vec<u32>;
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut total_calories = Vec::new();
        let mut tmp = 0;

        for line in raw_input.lines() {
            if line.is_empty() {
                total_calories.push(tmp);
                tmp = 0;
            } else {
                tmp += line.parse::<u32>().unwrap();
            }
        }

        total_calories.push(tmp);
        total_calories
    }

    fn part_1(total_calories: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(*total_calories.iter().max().unwrap_or(&0))
    }

    fn part_2(total_calories: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut total_calories = total_calories.clone();
        total_calories.sort_by(|a, b| b.cmp(a));
        Ok(total_calories[0] + total_calories[1] + total_calories[2])
    }
}
