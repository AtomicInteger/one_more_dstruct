pub mod stack;
pub mod queue;
pub mod vector_based_dstruct;

#[cfg(test)]
mod tests {

    use vector_based_dstruct::VectorBasedDataStructure;
    use stack::Stack;
    #[test]
    fn push_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn peek_stack() {
        let stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(stack.peek().unwrap(), 5);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn clear_stack() {
        let mut not_empty_stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(not_empty_stack.is_empty(), false);
        not_empty_stack.clear();
        assert_eq!(not_empty_stack.is_empty(), true);
    }

    #[test]
    fn is_empty_stack() {
        let not_empty_stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(not_empty_stack.is_empty(), false);
        assert_ne!(not_empty_stack.size(), 0);
        let empty_stack : Stack<i32> = Stack::new();
        assert_eq!(empty_stack.is_empty(), true);
        assert_eq!(empty_stack.size(), 0);
    }

    #[test]
    fn pop_stack() {
        let mut stack = Stack::from(vec!(1, 2, 3));
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn push_and_pop_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.size(), 0);
    }

    use queue::Queue;
    #[test]
    fn enqueue_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn dequeue_queue() {
        let mut queue = Queue::from(vec!(1, 2, 3));
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn peek_queue() {
        let queue = Queue::from(vec!(1, 2, 3));
        assert_eq!(queue.peek().unwrap(), 3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn clear_queue() {
        let mut not_empty_queue = Queue::from(vec!(1, 2, 5));
        assert_eq!(not_empty_queue.is_empty(), false);
        not_empty_queue.clear();
        assert_eq!(not_empty_queue.is_empty(), true);
    }

    #[test]
    fn is_empty_queue() {
        let not_empty_queue = Queue::from(vec!(1, 2, 5));
        assert_eq!(not_empty_queue.is_empty(), false);
        assert_ne!(not_empty_queue.size(), 0);
        let empty_queue : Queue<i32> = Queue::new();
        assert_eq!(empty_queue.is_empty(), true);
        assert_eq!(empty_queue.size(), 0);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
