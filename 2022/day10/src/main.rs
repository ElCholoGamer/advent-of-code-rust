use aoc_lib::{AocSolution, BoxedError};

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Self::NoOp
        } else {
            let split = s.split_whitespace().last().unwrap();
            Self::AddX(split.parse().unwrap())
        }
    }
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day10>(10)
}

struct Day10;

impl AocSolution for Day10 {
    type Input = Vec<Instruction>;
    type Output = String;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(Instruction::from).collect()
    }

    fn part_1(instructions: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut cycle = 0;
        let mut x_reg = 1;
        let mut next_signal_cycle = 20;
        let mut signal_sum = 0;

        for instruction in instructions {
            next_cycle(&mut cycle, &mut next_signal_cycle, &mut signal_sum, x_reg);
            if let Instruction::AddX(val) = instruction {
                next_cycle(&mut cycle, &mut next_signal_cycle, &mut signal_sum, x_reg);
                x_reg += val;
            }
        }

        Ok(signal_sum.to_string())
    }

    fn part_2(instructions: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut cycle = 0;
        let mut x_reg = 1;
        let mut pixels = [false; 240];

        for instruction in instructions {
            next_cycle_crt(&mut pixels, &mut cycle, x_reg);
            if let Instruction::AddX(val) = instruction {
                next_cycle_crt(&mut pixels, &mut cycle, x_reg);
                x_reg += val;
            }
        }

        let word = pixels.chunks(40)
            .map(|line_data| line_data.iter().map(|p| if *p { "██" } else { "  " }).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(format!("\n{}", word))
    }
}

fn next_cycle(cycle: &mut u32, next_signal_cycle: &mut u32, signal_sum: &mut i32, x_reg: i32) {
    *cycle += 1;
    if cycle >= next_signal_cycle {
        *next_signal_cycle += 40;
        *signal_sum += *cycle as i32 * x_reg;
    }
}

fn next_cycle_crt(pixels: &mut [bool; 240], cycle: &mut u32, x_reg: i32) {
    let pixel_x = (*cycle % 40) as i32;
    pixels[*cycle as usize] = pixel_x >= x_reg - 1 && pixel_x <= x_reg + 1;
    *cycle += 1;
}