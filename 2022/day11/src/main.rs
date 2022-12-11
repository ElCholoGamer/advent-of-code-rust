use aoc_lib::{AocSolution, BoxedError, Error};
use aoc_lib::utils::mcm;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Item(u64);

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_divisible: u64,
    throw_if_true: usize,
    throw_if_false: usize,
    inspect_count: u64,
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day11>(11)
}

struct Day11;

impl AocSolution for Day11 {
    type Input = Vec<Monkey>;
    type Output = u64;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.lines().collect::<Vec<_>>();
        lines.chunks(7).map(|lines| {
            let starting_items = lines[1]
                .split(": ")
                .last().unwrap()
                .split(", ")
                .map(|n| Item(n.parse().unwrap()))
                .collect::<Vec<_>>();
            let operation_words = lines[2].split(" = ").last().unwrap().split_whitespace().collect::<Vec<_>>();
            let operation = match operation_words[1] {
                "+" => Operation::Add(operation_words[2].parse().unwrap()),
                "*" if operation_words[2] == "old" => Operation::Square,
                "*" => Operation::Multiply(operation_words[2].parse().unwrap()),
                _ => panic!("invalid operation symbol"),
            };

            Monkey {
                items: starting_items,
                operation,
                test_divisible: lines[3].split_whitespace().last().unwrap().parse().unwrap(),
                throw_if_true: lines[4].split_whitespace().last().unwrap().parse().unwrap(),
                throw_if_false: lines[5].split_whitespace().last().unwrap().parse().unwrap(),
                inspect_count: 0,
            }
        }).collect()
    }

    fn part_1(monkeys: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(monkey_business(monkeys.clone(), 20, true))
    }

    fn part_2(monkeys: &Self::Input) -> Result<Self::Output, BoxedError> {
        Ok(monkey_business(monkeys.clone(), 10_000, false))
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: u32, divide_by_three: bool) -> u64 {
    let modulo_divisor = mcm(&monkeys.iter().map(|monkey| monkey.test_divisible).collect::<Vec<_>>());

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            monkeys[m].inspect_count += monkeys[m].items.len() as u64;

            for i in 0..monkeys[m].items.len() {
                let Item(mut worry_level) = monkeys[m].items[i];

                match monkeys[m].operation {
                    Operation::Add(val) => worry_level += val,
                    Operation::Multiply(val) => worry_level *= val,
                    Operation::Square => worry_level *= worry_level,
                }

                if divide_by_three { worry_level /= 3; }
                worry_level %= modulo_divisor;

                let target = if worry_level % monkeys[m].test_divisible == 0 {
                    monkeys[m].throw_if_true
                } else {
                    monkeys[m].throw_if_false
                };
                monkeys[target].items.push(Item(worry_level));
            }

            monkeys[m].items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys[0].inspect_count * monkeys[1].inspect_count
}