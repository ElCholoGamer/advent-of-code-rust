use std::ops::{Index, IndexMut, Range};
use std::iter;
use crate::Int;

#[derive(Debug)]
pub struct Memory {
    data: Vec<Int>
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    fn check_index(&mut self, index: usize) {
        let len = self.data.len();
        if index >= len {
            self.data.extend(iter::repeat(0).take(index - len))
        }
    }
}

impl From<&[Int]> for Memory {
    fn from(initial_data: &[Int]) -> Self {
        Self {
            data: Vec::from(initial_data)
        }
    }
}

impl From<&Vec<Int>> for Memory {
    fn from(initial_data: &Vec<Int>) -> Self {
        Self {
            data: initial_data.clone(),
        }
    }
}

impl Index<usize> for Memory {
    type Output = Int;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data.get(index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.check_index(index);
        &mut self.data[index]
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [Int];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range.start..range.end]
    }
}