extern crate core;

use vector_based_dstruct::VectorBasedDataStructure;

pub struct Stack<T> {
    entry : Vec<T>
}
impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.entry.push(value)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty()  {
            None
        } else {
            self.entry.get(self.size() - 1)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last_index = self.size() - 1;
            Some(self.entry.remove(last_index))
        }
    }
}
impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Stack<T> {
        Stack { entry : vec }
    }
}

impl<T> VectorBasedDataStructure<T> for Stack<T> {
    fn get_entry(&self) -> &Vec<T> {
        &self.entry
    }

    fn get_mut_entry(&mut self) -> &mut Vec<T> {
        &mut self.entry
    }

    fn new() -> Stack<T> {
        Stack { entry : Vec::new() }
    }
}
