pub mod stack;

#[cfg(test)]
mod tests {

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
        assert_eq!(stack.size(), 0)
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
