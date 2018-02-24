pub trait VectorBasedDataStructure<T> {
    fn size(&self) -> usize {
        self.get_entry().len()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn clear(&mut self) {
        self.get_mut_entry().clear()
    }

    fn new() -> Self;

    fn get_entry(&self) -> &Vec<T>;

    fn get_mut_entry(&mut self) -> &mut Vec<T>;
}
