extern crate core;

use vector_based_dstruct::VectorBasedDataStructure;

pub struct List<T> {
    entry: Vec<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        self.entry.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last_index = self.size() - 1;
            Some(self.entry.remove(last_index))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            return self.entry.get(self.size() - 1);
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            return self.entry.get(index);
        }
    }

    pub fn unshift(&mut self, value: T) {
        self.entry.insert(0, value)
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.entry.remove(0))
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> List<T> {
        List { entry: vec }
    }
}

impl<T> VectorBasedDataStructure<T> for List<T> {
    fn get_entry(&self) -> &Vec<T> {
        &self.entry
    }

    fn get_mut_entry(&mut self) -> &mut Vec<T> {
        &mut self.entry
    }

    fn new() -> List<T> {
        List { entry: Vec::new() }
    }
}
