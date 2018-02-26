use tree_node::TreeNode;

pub struct Tree<T> {
    root: Option<T>,
    children: Vec<Option<TreeNode<T>>>
}

impl<T: Clone> Tree<T> {
    pub fn get_root(&self) -> Option<T> {
        self.root.clone()
    }

    pub fn get_leaves(&self) -> Vec<TreeNode<T>> {
        let mut inner_leaves = vec![];
        for child in self.children.clone().iter() {
            inner_leaves.extend(self.search_leaves(&child.clone().unwrap()).iter().cloned());
        }
        inner_leaves
    }

    pub fn nodes(&self, parent_node : TreeNode<T>) -> Vec<TreeNode<T>> {
        let mut all_nodes = vec![parent_node.clone()];
        for node in parent_node.children().clone().iter() {
            all_nodes.push(node.clone().unwrap());
            if !node.clone().unwrap().children().is_empty() {
                let inner_nodes = self.nodes(node.clone().unwrap());
                all_nodes.extend(inner_nodes.iter().cloned());
            }
        }
        all_nodes
    }

    pub fn get_children(&self) -> Vec<Option<TreeNode<T>>> {
        self.children.clone()
    }

    fn search_leaves(&self, node: &TreeNode<T>) -> Vec<TreeNode<T>> {
        let mut result = vec![];
        if node.children().is_empty() {
            return vec![node.to_owned()]
        }
        for node_child in node.children() {
            let inner_leaves = self.search_leaves(&node_child.unwrap());
            result.extend(inner_leaves.iter().cloned());
        }
        result
    }

    pub fn new(root: T) -> Tree<T> {
        Tree {
            root: Some(root),
            children: vec![]
        }
    }

    pub fn new_with_children(root: T, children: Vec<Option<TreeNode<T>>>) -> Tree<T> {
        Tree {
            root: Some(root),
            children
        }
    }
}
