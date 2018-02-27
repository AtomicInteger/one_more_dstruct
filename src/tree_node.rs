#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    value: Option<T>,
    children: Vec<Option<TreeNode<T>>>,
}

impl<T: Clone> TreeNode<T> {
    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn get_children(&self) -> Vec<Option<TreeNode<T>>> {
        self.children.clone()
    }

    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value: Some(value),
            children: vec![],
        }
    }

    pub fn new_with_children(value: T, children: Vec<TreeNode<T>>) -> TreeNode<T> {
        let mut wrapped_children = vec![];
        children.iter().for_each(|node| {
            wrapped_children.push(Some(node.clone()));
        });
        TreeNode {
            value: Some(value),
            children: wrapped_children,
        }
    }
}

impl<T: PartialEq + Clone> PartialEq for TreeNode<T> {
    fn eq(&self, other: &TreeNode<T>) -> bool {
        self.value == other.value && self.children == other.get_children()
    }
}
