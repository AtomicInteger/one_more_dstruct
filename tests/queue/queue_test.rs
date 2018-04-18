extern crate one_more_dstruct as dstruct;

use self::dstruct::queue::queue::Queue;
use self::dstruct::vector_based_dstruct::VectorBasedDataStructure;

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
    let mut queue = Queue::from(vec![1, 2, 3]);
    assert_eq!(queue.dequeue().unwrap(), 3);
    assert_eq!(queue.size(), 2);
}

#[test]
fn peek_queue() {
    let queue = Queue::from(vec![1, 2, 3]);
    assert_eq!(queue.peek().unwrap().to_owned(), 3);
    assert_eq!(queue.size(), 3);
}

#[test]
fn clear_queue() {
    let mut not_empty_queue = Queue::from(vec![1, 2, 5]);
    assert_eq!(not_empty_queue.is_empty(), false);
    not_empty_queue.clear();
    assert_eq!(not_empty_queue.is_empty(), true);
}

#[test]
fn is_empty_queue() {
    let not_empty_queue = Queue::from(vec![1, 2, 5]);
    assert_eq!(not_empty_queue.is_empty(), false);
    assert_ne!(not_empty_queue.size(), 0);
    let empty_queue: Queue<i32> = Queue::new();
    assert_eq!(empty_queue.is_empty(), true);
    assert_eq!(empty_queue.size(), 0);
}