extern crate core;

use self::core::ptr;

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

    pub fn peek(&self) -> Option<T> {
        if self.is_empty()  {
            None
        } else {
            unsafe {
                Some(ptr::read(self.entry.get(self.size() - 1).unwrap()))
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

    pub fn from(vec : Vec<T>) -> Queue<T> {
        Queue { entry : vec }
    }

    pub fn new() -> Queue<T> {
        Queue { entry : Vec::new() }
    }
}