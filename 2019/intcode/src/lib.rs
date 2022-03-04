mod memory;
mod error;

use std::ops::{Index, IndexMut};
use std::fs;

pub use crate::memory::Memory;
pub use crate::error::{Error, Result};

pub type Int = i64;

#[derive(Debug)]
enum Parameter {
    Position(Int),
}

#[derive(Debug)]
pub struct IntcodeProgram {
    pub memory: Memory,
    ip: usize,
    state: ProgramState,
}

#[derive(Debug)]
pub enum ProgramState {
    Running,
    Output(Int),
    Halted,
}

impl IntcodeProgram {
    pub fn new(program: &[Int]) -> Self {
        Self {
            memory: Memory::from(program),
            ip: 0,
            state: ProgramState::Halted
        }
    }

    pub fn from_raw(data: &str) -> Result<Self> {
        let ints: Vec<Int> = data.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self::new(&ints))
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path).map_err(|e| Error::from(e.to_string()))?;
        Self::from_raw(contents.as_str())
    }

    pub fn step(&mut self) -> Result<()> {
        let instruction = self.fetch();
        let (opcode, params) = self.decode(instruction)?;

        self.ip += 1 + params.len();
        self.execute(opcode, &params)?;

        Ok(())
    }

    pub fn state(&self) -> &ProgramState {
        &self.state
    }

    pub fn run(&mut self) -> Result<()> {
        self.state = ProgramState::Running;

        loop {
            self.step()?;

            if let ProgramState::Halted = self.state {
                break Ok(());
            }
        }
    }

    fn fetch(&self) -> Int {
        self.memory[self.ip]
    }

    fn decode(&self, instruction: Int) -> Result<(Int, Vec<Parameter>)> {
        let opcode = instruction;

        let arg_count = match opcode {
            1 | 2 => 3,
            99 => 0,
            _ => return Err(Error::from(format!("Invalid opcode: {}", opcode))),
        };

        let start = self.ip + 1;
        let param_ints = &self.memory[start..(start + arg_count)];
        let params = param_ints.into_iter().map(|i| Parameter::Position(*i)).collect();

        Ok((opcode, params))
    }

    fn execute(&mut self, opcode: Int, params: &[Parameter]) -> Result<()> {
        match opcode {
            1 => self[&params[2]] = self[&params[0]] + self[&params[1]],
            2 => self[&params[2]] = self[&params[0]] * self[&params[1]],
            99 => self.state = ProgramState::Halted,
            _ => return Err(Error::from(format!("Invalid opcode: {}", opcode))),
        }

        Ok(())
    }
}

impl Index<&Parameter> for IntcodeProgram {
    type Output = Int;

    fn index(&self, index: &Parameter) -> &Self::Output {
        match index {
            Parameter::Position(addr) => &self.memory[*addr as usize],
        }
    }
}

impl IndexMut<&Parameter> for IntcodeProgram {
    fn index_mut(&mut self, index: &Parameter) -> &mut Self::Output {
        match index {
            Parameter::Position(addr) => &mut self.memory[*addr as usize]
        }
    }
}