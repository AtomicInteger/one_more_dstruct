extern crate one_more_dstruct as dstruct;

use self::dstruct::list::list::List;
use self::dstruct::vector_based_dstruct::VectorBasedDataStructure;

#[test]
fn push_list() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.size(), 3);
}

#[test]
fn pop_list() {
    let mut list = List::from(vec![1, 2, 3, 4]);
    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.size(), 3);
}

#[test]
fn peek_list() {
    let list = List::from(vec![1, 2, 3, 4]);
    assert_eq!(list.peek().unwrap().to_owned(), 4);
    assert_eq!(list.size(), 4);
}

#[test]
fn unshift_list() {
    let mut list = List::new();
    list.unshift(1);
    list.unshift(2);
    list.unshift(3);
    assert_eq!(list.size(), 3);
}

#[test]
fn shift_list() {
    let mut list = List::from(vec![1, 2, 3, 4]);
    assert_eq!(list.shift().unwrap(), 1);
    assert_eq!(list.size(), 3);
}

#[test]
fn get_list() {
    let list = List::from(vec![1, 2, 3, 4]);
    assert_eq!(list.get(0).unwrap().to_owned(), 1);
    assert_eq!(list.get(2).unwrap().to_owned(), 3);
    assert_eq!(list.get(3).unwrap().to_owned(), 4);
}