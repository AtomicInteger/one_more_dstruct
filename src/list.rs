extern crate core;

use self::core::ptr;
use vector_based_dstruct::VectorBasedDataStructure;

pub struct List<T> {
    entry : Vec<T>
}

impl<T> List<T> {
    pub fn push(&mut self, value : T) {
        self.entry.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last_index = self.size() - 1;
            unsafe {
                let elem = ptr::read(self.entry.get(last_index).unwrap());
                self.entry.remove(last_index);
                Some(elem)
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty()  {
            None
        } else {
            unsafe {
                Some(ptr::read(self.entry.get(self.size() - 1).unwrap()))
            }
        }
    }

    pub fn get(&self, index : usize) -> Option<T> {
        if self.is_empty()  {
            None
        } else {
            unsafe {
                Some(ptr::read(self.entry.get(index).unwrap()))
            }
        }
    }

    pub fn unshift(&mut self, value : T) {
        self.entry.insert(0, value)
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                let elem = ptr::read(self.entry.get(0).unwrap());
                self.entry.remove(0);
                Some(elem)
            }
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> List<T> {
        List { entry : vec }
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
        List { entry : Vec::new() }
    }
}
