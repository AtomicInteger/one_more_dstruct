extern crate core;

use self::core::ptr;

pub struct Stack<T> {
    entry : Vec<T>
}
impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.entry.push(value)
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

    pub fn size(&self) -> usize {
        self.entry.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn clear(&mut self) {
        self.entry.clear()
    }

    pub fn from(vec : Vec<T>) -> Stack<T> {
        Stack { entry : vec }
    }

    pub fn new() -> Stack<T> {
        Stack { entry : Vec::new() }
    }
}
