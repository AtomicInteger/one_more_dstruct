extern crate one_more_dstruct as dstruct;

use self::dstruct::queue::queue::Queue;
use self::dstruct::vector_based_dstruct::VectorBasedDataStructure;

#[test]
fn enqueue_and_dequeue() {
    let mut queue = Queue::from(vec![1, 2, 3]);
    assert_eq!(queue.size(), 3);
    queue.enqueue(4);
    assert_eq!(queue.size(), 4);
    assert_eq!(queue.get_entry(), &vec![4, 1, 2, 3]);
    queue.enqueue(5);
    assert_eq!(queue.size(), 5);
    assert_eq!(queue.get_entry(), &vec![5, 4, 1, 2, 3]);
    assert_eq!(queue.dequeue().unwrap(), 3);
    assert_eq!(queue.get_entry(), &vec![5, 4, 1, 2]);
    assert_eq!(queue.size(), 4);
    assert_eq!(queue.dequeue().unwrap(), 2);
    assert_eq!(queue.get_entry(), &vec![5, 4, 1]);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.dequeue().unwrap(), 1);
    assert_eq!(queue.get_entry(), &vec![5, 4]);
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.dequeue().unwrap(), 4);
    assert_eq!(queue.get_entry(), &vec![5]);
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.dequeue().unwrap(), 5);
    assert_eq!(queue.get_entry(), &vec![]);
    assert!(queue.is_empty());
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}

#[test]
fn enqueue_and_peek() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.peek().unwrap(), &1);
    queue.enqueue(2);
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.peek().unwrap(), &1);
    assert_eq!(queue.size(), 2);
}

#[test]
fn enqueue_and_clear() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    assert_eq!(queue.size(), 1);
    queue.enqueue(2);
    assert_eq!(queue.size(), 2);
    queue.clear();
    assert!(queue.is_empty());
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(5);
    assert_eq!(queue.size(), 3);
    queue.clear();
    assert!(queue.is_empty());
}

#[test]
fn dequeue_and_peek() {
    let mut queue = Queue::from(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(queue.size(), 6);
    assert_eq!(queue.peek().unwrap(), &6);
    assert_eq!(queue.size(), 6);
    assert_eq!(queue.dequeue().unwrap(), 6);
    assert_eq!(queue.size(), 5);
    assert_eq!(queue.peek().unwrap(), &5);
    assert_eq!(queue.size(), 5);
    assert_eq!(queue.dequeue().unwrap(), 5);
    assert_eq!(queue.size(), 4);
    assert_eq!(queue.peek().unwrap(), &4);
    assert_eq!(queue.size(), 4);
    assert_eq!(queue.dequeue().unwrap(), 4);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek().unwrap(), &3);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.dequeue().unwrap(), 3);
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.peek().unwrap(), &2);
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.dequeue().unwrap(), 2);
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.peek().unwrap(), &1);
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.dequeue().unwrap(), 1);
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.peek(), None);
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.dequeue(), None);
    assert_eq!(queue.size(), 0);
}