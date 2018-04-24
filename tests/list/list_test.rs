extern crate one_more_dstruct as dstruct;

use self::dstruct::list::list::List;
use self::dstruct::vector_based_dstruct::VectorBasedDataStructure;

#[test]
fn push_and_pop() {
    let mut list = List::from(vec![1, 2, 3]);
    assert_eq!(list.size(), 3);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get_entry(), &vec![1, 2]);
    list.push(4);
    assert_eq!(list.size(), 3);
    assert_eq!(list.get_entry(), &vec![1, 2, 4]);
    list.push(5);
    assert_eq!(list.size(), 4);
    assert_eq!(list.get_entry(), &vec![1, 2, 4, 5]);
    list.push(6);
    assert_eq!(list.size(), 5);
    assert_eq!(list.get_entry(), &vec![1, 2, 4, 5, 6]);
    assert_eq!(list.pop().unwrap(), 6);
    assert_eq!(list.size(), 4);
    assert_eq!(list.pop().unwrap(), 5);
    assert_eq!(list.size(), 3);
    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.size(), 2);
    assert_eq!(list.pop().unwrap(), 2);
    assert_eq!(list.size(), 1);
    assert_eq!(list.pop().unwrap(), 1);
    assert!(list.is_empty());
    assert_eq!(list.pop(), None);
    assert!(list.is_empty());
}

#[test]
fn push_and_peek() {
    let mut list = List::new();
    assert!(list.is_empty());
    assert_eq!(list.peek(), None);
    list.push(0);
    assert_eq!(list.size(), 1);
    assert_eq!(list.peek().unwrap(), &0);
    assert_eq!(list.size(), 1);
}

#[test]
fn push_and_get() {
    let mut list = List::new();
    assert!(list.is_empty());
    assert_eq!(list.get(0), None);
    list.push(1);
    assert_eq!(list.size(), 1);
    list.push(2);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get(0).unwrap(), &1);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get(1).unwrap(), &2);
    assert_eq!(list.size(), 2);
}

#[test]
fn push_and_unshift() {
    let mut list = List::new();
    assert!(list.is_empty());
    list.push(0);
    assert_eq!(list.size(), 1);
    list.unshift(1);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get_entry(), &vec![1, 0]);
    list.push(3);
    assert_eq!(list.size(), 3);
    assert_eq!(list.get_entry(), &vec![1, 0, 3]);
    list.unshift(4);
    assert_eq!(list.size(), 4);
    assert_eq!(list.get_entry(), &vec![4, 1, 0, 3]);
}

#[test]
fn push_and_shift() {
    let mut list = List::from(vec![1, 2, 3]);
    assert_eq!(list.size(), 3);
    assert_eq!(list.shift().unwrap(), 1);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get_entry(), &vec![2, 3]);
    list.push(4);
    assert_eq!(list.size(), 3);
    assert_eq!(list.get_entry(), &vec![2, 3, 4]);
    assert_eq!(list.shift().unwrap(), 2);
    assert_eq!(list.size(), 2);
    assert_eq!(list.get_entry(), &vec![3, 4]);
    assert_eq!(list.shift().unwrap(), 3);
    assert_eq!(list.size(), 1);
    assert_eq!(list.get_entry(), &vec![4]);
    assert_eq!(list.shift().unwrap(), 4);
    assert!(list.is_empty());
    assert_eq!(list.shift(), None);
    assert!(list.is_empty());
    list.push(5);
    assert_eq!(list.size(), 1);
    assert_eq!(list.shift().unwrap(), 5);
    assert!(list.is_empty());
}

#[test]
fn shift_and_unshift() {
    let mut list = List::from(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.size(), 5);
    assert_eq!(list.shift().unwrap(), 1);
    assert_eq!(list.size(), 4);
    assert_eq!(list.get_entry(), &vec![2, 3, 4, 5]);
    list.unshift(9);
    assert_eq!(list.size(), 5);
    assert_eq!(list.get_entry(), &vec![9, 2, 3, 4, 5]);
    assert_eq!(list.shift().unwrap(), 9);
    assert_eq!(list.size(), 4);
    assert_eq!(list.shift().unwrap(), 2);
    assert_eq!(list.size(), 3);
    assert_eq!(list.shift().unwrap(), 3);
    assert_eq!(list.size(), 2);
    assert_eq!(list.shift().unwrap(), 4);
    assert_eq!(list.size(), 1);
    assert_eq!(list.shift().unwrap(), 5);
    assert!(list.is_empty());
    assert_eq!(list.shift(), None);
    assert!(list.is_empty());
    list.unshift(1);
    assert_eq!(list.size(), 1);
    list.unshift(2);
    assert_eq!(list.size(), 2);
    list.unshift(3);
    assert_eq!(list.size(), 3);
    list.unshift(4);
    assert_eq!(list.size(), 4);
    list.unshift(5);
    assert_eq!(list.size(), 5);
    assert_eq!(list.shift().unwrap(), 5);
    assert_eq!(list.size(), 4);
}