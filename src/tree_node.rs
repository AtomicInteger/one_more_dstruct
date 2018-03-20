#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    value: Option<T>,
    children: Vec<TreeNode<T>>,
}

impl<T: Clone> TreeNode<T> {
    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn get_children(&self) -> Vec<TreeNode<T>> {
        self.children.clone()
    }

    pub fn get_mut_children(&mut self) -> &mut Vec<TreeNode<T>> {
        &mut self.children
    }

    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value: Some(value),
            children: vec![],
        }
    }

    pub fn new_with_children(value: T, children: Vec<TreeNode<T>>) -> TreeNode<T> {
        TreeNode {
            value: Some(value),
            children,
        }
    }
}

impl<T: PartialEq + Clone> PartialEq for TreeNode<T> {
    fn eq(&self, other: &TreeNode<T>) -> bool {
        self.value == other.value && self.children == other.get_children()
    }
}
