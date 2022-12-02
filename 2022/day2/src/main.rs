use aoc_lib::{BoxedError, GenericError};

#[derive(Debug)]
struct Instruction {
    play: Play,
    enemy_play: Play,
}

#[derive(Debug, PartialEq, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn winning_play(&self) -> Play {
        match self {
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
        }
    }

    fn score_value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = BoxedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(' ').expect("invalid instruction");
        let enemy_play = match a {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => return Err(GenericError::from("Invalid first play").into()),
        };
        let play = match b {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => return Err(GenericError::from("Invalid second play").into()),
        };

        Ok(Self { play, enemy_play })
    }
}

fn main() -> Result<(), BoxedError> {
    let instructions = aoc_lib::get_input(2, |l| l.parse::<Instruction>())?;

    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
    Ok(())
}

fn part_1(instructions: &[Instruction]) -> u32 {
    let mut score = 0;

    for Instruction { play, enemy_play } in instructions {
        if *play == enemy_play.winning_play() {
            score += 6; // Victory
        } else if play == enemy_play {
            score += 3; // Draw
        }

        score += play.score_value();
    }

    score
}

fn part_2(instructions: &[Instruction]) -> u32 {
    let mut score = 0;

    for instruction in instructions {
        let (result_score, play) = match instruction.play {
            Play::Rock => (0, instruction.enemy_play.winning_play().winning_play()),
            Play::Paper => (3, instruction.enemy_play.clone()),
            Play::Scissors => (6, instruction.enemy_play.winning_play()),
        };

        score += play.score_value() + result_score;
    }

    score
}