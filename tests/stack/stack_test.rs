extern crate one_more_dstruct as dstruct;

use self::dstruct::stack::stack::Stack;
use self::dstruct::vector_based_dstruct::VectorBasedDataStructure;

#[test]
fn push_and_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(stack.size(), 1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.pop().unwrap(), 1);
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.size(), 0);
}

#[test]
fn push_and_peek() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek().unwrap(), &1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek().unwrap(), &2);
    assert_eq!(stack.size(), 2);
}

#[test]
fn push_and_clear() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(stack.size(), 1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    stack.clear();
    assert!(stack.is_empty());
    stack.push(3);
    stack.push(4);
    stack.push(5);
    assert_eq!(stack.size(), 3);
    stack.clear();
    assert!(stack.is_empty());
}

#[test]
fn peek_and_pop() {
    let mut stack = Stack::from(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.peek().unwrap(), &6);
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.pop().unwrap(), 6);
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.peek().unwrap(), &5);
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.pop().unwrap(), 5);
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.peek().unwrap(), &4);
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.pop().unwrap(), 4);
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek().unwrap(), &3);
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop().unwrap(), 3);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek().unwrap(), &2);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek().unwrap(), &1);
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.pop().unwrap(), 1);
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.size(), 0);
}