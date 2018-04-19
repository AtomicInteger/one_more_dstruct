use vector_based_dstruct::VectorBasedDataStructure;

pub struct Stack<T> {
    entry: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.entry.push(value)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
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
        Stack { entry: vec }
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
        Stack { entry: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    use vector_based_dstruct::VectorBasedDataStructure;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(0);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
        stack.push(1);
        assert_eq!(stack.size(), 2);
        stack.push(2);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(stack.size(), 9);
        let last = stack.pop().unwrap();
        assert_eq!(stack.size(), 8);
        assert_eq!(last, 9);
        let last = stack.pop().unwrap();
        assert_eq!(stack.size(), 7);
        assert_eq!(last, 8);
        let mut empty_stack: Stack<i32> = Stack::new();
        assert!(empty_stack.is_empty());
        let last = empty_stack.pop();
        assert_eq!(last, None);
    }

    #[test]
    fn peek() {
        let stack = Stack::from(vec![1, 2, 3]);
        assert_eq!(stack.size(), 3);
        let last = stack.peek().unwrap();
        assert_eq!(stack.size(), 3);
        assert_eq!(last, &3);
        let empty_stack : Stack<i32> = Stack::new();
        assert_eq!(empty_stack.size(), 0);
        let last = empty_stack.peek();
        assert_eq!(last, None);
    }

    #[test]
    fn clear() {
        let mut stack = Stack::from(vec![1, 2, 3]);
        assert!(!stack.is_empty());
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn is_empty() {
        let stack = Stack::from(vec![1, 2, 3]);
        assert!(!stack.is_empty());
        assert_ne!(stack.size(), 0);
        let empty_stack: Stack<i32> = Stack::new();
        assert!(empty_stack.is_empty());
        assert_eq!(empty_stack.size(), 0);
    }
}
