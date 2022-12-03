use aoc_lib::BoxedError;

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

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (a, b) = s.split_once(' ').expect("invalid instruction");
        let enemy_play = match a {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid first play"),
        };
        let play = match b {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Invalid second play"),
        };

        Self { play, enemy_play }
    }
}

fn main() -> Result<(), BoxedError> {
    let instructions = aoc_lib::get_input(2, |l| Instruction::from(l))?;

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