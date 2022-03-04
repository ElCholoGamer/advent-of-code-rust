use intcode::{Error, Int, IntcodeProgram, Result};

fn main() {
    println!("Part 1: {}", part_1().unwrap());
    println!("Part 2: {}", part_2().unwrap());
}

fn part_1() -> Result<Int> {
    let mut program = IntcodeProgram::from_file("inputs/day2.txt")?;
    program.memory[1] = 12;
    program.memory[2] = 2;

    program.run()?;

    Ok(program.memory[0])
}

fn part_2() -> Result<Int> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = IntcodeProgram::from_file("inputs/day2.txt")?;
            program.memory[1] = noun;
            program.memory[2] = verb;

            program.run()?;

            if program.memory[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(Error::from("Could not find a valid noun-verb combination"))
}