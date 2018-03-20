use tree_node::TreeNode;

pub struct Tree<T> {
    root: TreeNode<T>,
}

impl<T: Clone + PartialEq> Tree<T> {
    pub fn get_root(&self) -> TreeNode<T> {
        self.root.clone()
    }

    pub fn get_mut_root(&mut self) -> &mut TreeNode<T> {
        &mut self.root
    }

    pub fn get_leaves(&self) -> Vec<TreeNode<T>> {
        let mut inner_leaves = vec![];
        for child in self.get_root().get_children().clone().iter() {
            inner_leaves.extend(self.search_leaves(&child.clone()).iter().cloned());
        }
        inner_leaves
    }

    pub fn nodes(&self, parent_node: TreeNode<T>) -> Vec<TreeNode<T>> {
        let mut all_nodes = vec![parent_node.clone()];
        for node in parent_node.get_children().clone().iter() {
            all_nodes.push(node.clone());
            if !node.clone().get_children().is_empty() {
                let inner_nodes = self.nodes(node.clone());
                all_nodes.extend(inner_nodes.iter().cloned());
            }
        }
        all_nodes
    }

    pub fn get_children(&self) -> Vec<TreeNode<T>> {
        self.get_root().get_children().clone()
    }

    fn search_leaves(&self, node: &TreeNode<T>) -> Vec<TreeNode<T>> {
        let mut result = vec![];
        if node.get_children().is_empty() {
            return vec![node.to_owned()];
        }
        for node_child in node.get_children() {
            let inner_leaves = self.search_leaves(&node_child);
            result.extend(inner_leaves.iter().cloned());
        }
        result
    }

    pub fn get_by_val(&self, value: T) -> Option<TreeNode<T>> {
        for node in self.nodes(self.get_root()) {
            if node.get_value().unwrap() == value {
                return Some(node);
            }
        }
        None
    }

    pub fn get_parent_by_val(&self, val: T) -> TreeNode<T> {
        for node in self.nodes(self.get_root()) {
            if node.get_children()
                .iter()
                .any(|child| child.clone().get_value().unwrap() == val)
            {
                return node.clone();
            }
        }
        panic!("Cannot get node's parent!");
    }

    pub fn add_root_sub_tree(&mut self, sub_tree: Tree<T>) {
        self.get_mut_root()
            .get_mut_children()
            .push(sub_tree.get_root());
    }

    pub fn new(root: TreeNode<T>) -> Tree<T> {
        Tree { root }
    }

    pub fn new_with_children(root_value: T, children: Vec<TreeNode<T>>) -> Tree<T> {
        Tree {
            root: TreeNode::new_with_children(root_value, children),
        }
    }
}
