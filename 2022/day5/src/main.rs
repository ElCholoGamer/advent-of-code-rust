use aoc_lib::{AocSolution, BoxedError};

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day5>(5)
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let words = s.split_whitespace().collect::<Vec<_>>();
        Self {
            amount: words[1].parse().unwrap(),
            from: words[3].parse::<usize>().unwrap() - 1,
            to: words[5].parse::<usize>().unwrap() - 1,
        }
    }
}

#[derive(Debug)]
struct CrateInput {
    initial_setup: Vec<Vec<char>>,
    procedure: Vec<Instruction>,
}

struct Day5;

impl AocSolution for Day5 {
    type Input = CrateInput;
    type Output = String;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.lines().collect::<Vec<_>>();
        let separator = lines.iter().position(|l| l.is_empty()).unwrap();

        let pile_count = lines[separator - 1].split_whitespace().count();
        let mut initial_setup = vec![Vec::new(); pile_count];

        for &line in lines[..separator - 1].iter().rev() {
            for i in 0..pile_count {
                let char = line.chars().nth(1 + i * 4).expect("row too small");
                if char != ' ' {
                    initial_setup[i].push(char);
                }
            }
        }

        CrateInput {
            procedure: lines[(separator + 1)..].iter().map(|&s| Instruction::from(s)).collect(),
            initial_setup,
        }
    }

    fn part_1(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut piles = input.initial_setup.clone();
        for instruction in input.procedure.iter() {
            for _ in 0..instruction.amount {
                let item = piles[instruction.from].pop().expect("missing item on pile");
                piles[instruction.to].push(item);
            }
        }

        Ok(piles.iter().map(|pile| pile.last().unwrap_or(&' ')).collect::<String>())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut piles = input.initial_setup.clone();

        for &Instruction { from, to, amount } in input.procedure.iter() {
            let mut items = piles[from].drain((piles[from].len() - amount)..).collect();
            piles[to].append(&mut items);
        }

        Ok(piles.iter().map(|pile| pile.last().unwrap_or(&' ')).collect::<String>())
    }
}
