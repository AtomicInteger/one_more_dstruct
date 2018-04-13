use tree::Tree;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    value: Option<T>,
    children: Vec<TreeNode<T>>,
}

impl<T: PartialEq + Clone> TreeNode<T> {
    pub fn get_value(&self) -> &Option<T> {
        &self.value
    }

    pub fn get_children(&self) -> &Vec<TreeNode<T>> {
        &self.children
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

    pub fn add_sub_tree(&mut self, sub_tree: Tree<T>) {
        self.get_mut_children().push(sub_tree.get_owned_root());
    }

    pub fn get_mut_by_val(&mut self, value: T) -> Option<&mut TreeNode<T>> {
        for node in self.get_mut_children().iter_mut() {
            if node.get_value().clone().unwrap() == value {
                return Some(node);
            }
            if !node.get_children().is_empty() {
                return node.get_mut_by_val(value);
            }
        }
        None
    }

    pub fn get_mut_parent_by_val(&mut self, value: &T) -> &mut TreeNode<T> {
        if self.get_children().contains(&TreeNode::new(value.clone())) {
            return self;
        }
        for child in self.get_mut_children().iter_mut() {
            return child.get_mut_parent_by_val(&value);
        }
        panic!("Cannot get node's parent!");
    }
}
