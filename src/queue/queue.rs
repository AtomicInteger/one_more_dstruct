extern crate core;

use vector_based_dstruct::VectorBasedDataStructure;

pub struct Queue<T> {
    entry: Vec<T>,
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, value: T) {
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
        Queue { entry: vec }
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
        Queue { entry: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use vector_based_dstruct::VectorBasedDataStructure;

    #[test]
    fn enqueue() {
        let mut queue = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.size(), 3);
        queue.enqueue(4);
        assert_eq!(queue.size(), 4);
        queue.enqueue(5);
        assert_eq!(queue.size(), 5);
        queue.enqueue(6);
        assert_eq!(queue.size(), 6);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn peek() {
        let queue = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.peek().unwrap(), &3);
        assert_eq!(queue.size(), 3);
        let empty_queue : Queue<i32> = Queue::new();
        assert!(empty_queue.is_empty());
        assert_eq!(empty_queue.peek(), None)
    }

    #[test]
    fn clear() {
        let mut queue = Queue::from(vec![1, 2, 3]);
        assert!(!queue.is_empty());
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn is_empty() {
        let queue = Queue::from(vec![1, 2, 3]);
        assert!(!queue.is_empty());
        assert_ne!(queue.size(), 0);
        let empty_queue: Queue<i32> = Queue::new();
        assert!(empty_queue.is_empty());
        assert_eq!(empty_queue.size(), 0);
    }
}
