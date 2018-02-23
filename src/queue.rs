extern crate core;

use vector_based_dstruct::VectorBasedDataStructure;

pub struct Queue<T> {
    entry : Vec<T>
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, value : T) {
        self.entry.push(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.entry.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.entry.get(self.size() - 1)
        }
    }
}
impl<T> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Queue<T> {
        Queue { entry : vec }
    }
}

impl<T> VectorBasedDataStructure<T> for Queue<T> {
    fn get_entry(&self) -> &Vec<T> {
        &self.entry
    }

    fn get_mut_entry(&mut self) -> &mut Vec<T> {
        &mut self.entry
    }

    fn new() -> Self {
        Queue { entry : Vec::new() }
    }
}