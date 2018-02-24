use tree_node::TreeNode;

pub struct Tree<T> {
    root : Option<T>,
    children : (Option<TreeNode<T>>, Option<TreeNode<T>>)
}

impl<T : Clone> Tree<T> {
    pub fn get_root(&self) -> Option<T> {
        self.root.clone()
    }

    pub fn get_leaf(&self) -> Vec<TreeNode<T>> {
        let mut left_leaves = self.search_leaf(&self.children.0.clone().unwrap());
        let right_leaves = self.search_leaf(&self.children.1.clone().unwrap());
        left_leaves.extend(right_leaves.iter().cloned());
        left_leaves
    }

    fn search_leaf(&self, node : &TreeNode<T>) -> Vec<TreeNode<T>> {
        let mut result = vec!();
        if node.children().0.is_none() && node.children().1.is_none()  {
            return vec!(node.to_owned());
        }
        let left_leaves = self.search_leaf(&node.children().0.unwrap());
        let right_leaves = self.search_leaf(&node.children().0.unwrap());
        result.extend(left_leaves.iter().cloned());
        result.extend(right_leaves.iter().cloned());
        result
    }

    pub fn new(root : T) -> Tree<T> {
        Tree{ root : Some(root), children : (None, None)}
    }

    pub fn new_with_children(root : T, children : (Option<TreeNode<T>>, Option<TreeNode<T>>)) -> Tree<T> {
        Tree{ root : Some(root), children}
    }
}