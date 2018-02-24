#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    value : Option<T>,
    children : Box<(Option<TreeNode<T>>, Option<TreeNode<T>>)>
}

impl<T : Clone> TreeNode<T> {
    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn children(&self) -> (Option<TreeNode<T>>, Option<TreeNode<T>>) {
        *self.children.clone()
    }

    pub fn new(value : T) -> TreeNode<T> {
        TreeNode {value : Some(value), children : Box::from((None, None))}
    }
}

impl<T : PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &TreeNode<T>) -> bool {
        self.value == other.value
    }
}