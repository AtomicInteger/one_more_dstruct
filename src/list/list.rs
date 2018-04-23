extern crate core;

use vector_based_dstruct::VectorBasedDataStructure;

pub struct List<T> {
    entry: Vec<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        self.entry.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last_index = self.size() - 1;
            Some(self.entry.remove(last_index))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            return self.entry.get(self.size() - 1);
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            return self.entry.get(index);
        }
    }

    pub fn unshift(&mut self, value: T) {
        self.entry.insert(0, value)
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.entry.remove(0))
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> List<T> {
        List { entry: vec }
    }
}

impl<T> VectorBasedDataStructure<T> for List<T> {
    fn get_entry(&self) -> &Vec<T> {
        &self.entry
    }

    fn get_mut_entry(&mut self) -> &mut Vec<T> {
        &mut self.entry
    }

    fn new() -> List<T> {
        List { entry: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use vector_based_dstruct::VectorBasedDataStructure;

    #[test]
    fn push() {
        let mut list = List::from(vec![1, 2, 3]);
        assert_eq!(list.size(), 3);
        list.push(4);
        assert_eq!(list.size(), 4);
        list.push(5);
        assert_eq!(list.size(), 5);
    }

    #[test]
    fn pop() {
        let mut list = List::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.size(), 5);
        assert_eq!(list.pop().unwrap(), 5);
        assert_eq!(list.size(), 4);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.size(), 3);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.size(), 2);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop().unwrap(), 1);
        assert!(list.is_empty());
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let list = List::from(vec![1, 2, 3]);
        assert_eq!(list.size(), 3);
        assert_eq!(list.peek().unwrap(), &3);
        assert_eq!(list.size(), 3);
        let empty_list : List<i32> = List::new();
        assert!(empty_list.is_empty());
        assert_eq!(empty_list.peek(), None);
        assert!(empty_list.is_empty());
    }

    #[test]
    fn get() {
        let list = List::from(vec![1, 2, 3]);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get(0).unwrap(), &1);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get(1).unwrap(), &2);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get(2).unwrap(), &3);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get(1000), None);
        assert_eq!(list.size(), 3);
        let empty_list : List<i32> = List::new();
        assert!(empty_list.is_empty());
        assert_eq!(empty_list.get(0), None);
        assert!(empty_list.is_empty());
    }

    #[test]
    fn unshift() {
        let mut list = List::from(vec![1, 2, 3]);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get_entry(), &vec![1, 2, 3]);
        list.unshift(4);
        assert_eq!(list.size(), 4);
        assert_eq!(list.get_entry(), &vec![4, 1, 2, 3]);
        list.unshift(5);
        assert_eq!(list.size(), 5);
        assert_eq!(list.get_entry(), &vec![5, 4, 1, 2, 3]);
    }

    #[test]
    fn shift() {
        let mut list = List::from(vec![1, 2, 3]);
        assert_eq!(list.size(), 3);
        assert_eq!(list.get_entry(), &vec![1, 2, 3]);
        assert_eq!(list.shift().unwrap(), 1);
        assert_eq!(list.size(), 2);
        assert_eq!(list.get_entry(), &vec![2, 3]);
        assert_eq!(list.shift().unwrap(), 2);
        assert_eq!(list.size(), 1);
        assert_eq!(list.get_entry(), &vec![3]);
        assert_eq!(list.shift().unwrap(), 3);
        assert_eq!(list.size(), 0);
        assert_eq!(list.get_entry(), &vec![]);
        assert_eq!(list.shift(), None);
    }
}