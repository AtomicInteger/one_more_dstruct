use tree_node::TreeNode;

pub struct Tree<T> {
    root: TreeNode<T>,
}

impl<T: Clone + PartialEq> Tree<T> {
    pub fn get_root(&self) -> &TreeNode<T> {
        &self.root
    }

    pub fn get_owned_root(self) -> TreeNode<T> {
        self.root
    }

    pub fn get_mut_root(&mut self) -> &mut TreeNode<T> {
        &mut self.root
    }

    pub fn get_leaves(&self) -> Vec<&TreeNode<T>> {
        Vec::from(self.search_leaves(self.get_root()))
    }

    pub fn nodes<'a>(&'a self, parent_node: &'a TreeNode<T>) -> Vec<&'a TreeNode<T>> {
        let mut all_nodes = vec![parent_node];
        for node in parent_node.get_children().iter() {
            all_nodes.push(node);
            if !node.get_children().is_empty() {
                let inner_nodes = self.nodes(node);
                all_nodes.extend(inner_nodes.iter());
            }
        }
        all_nodes
    }

    pub fn get_children(&self) -> &Vec<TreeNode<T>> {
        self.get_root().get_children()
    }

    fn search_leaves<'a>(&'a self, node: &'a TreeNode<T>) -> Vec<&'a TreeNode<T>> {
        if node.get_children().is_empty() {
            return vec![node];
        }
        let mut result = vec![];
        for node_child in node.get_children() {
            let inner_leaves = self.search_leaves(node_child);
            result.extend(inner_leaves.iter());
        }
        result
    }

    pub fn get_by_val(&self, value: T) -> Option<&TreeNode<T>> {
        for node in self.nodes(self.get_root()) {
            if node.get_value().clone().unwrap() == value {
                return Some(node);
            }
        }
        None
    }

    pub fn get_parent_by_val(&self, val: T) -> &TreeNode<T> {
        for node in self.nodes(self.get_root()) {
            if node.get_children().iter().any(|child| child.get_value().clone().unwrap() == val) {
                return node;
            }
        }
        panic!("Cannot get node's parent!");
    }

    pub fn add_root_sub_tree(&mut self, sub_tree: Tree<T>) {
        self.get_mut_root().get_mut_children().push(sub_tree.get_owned_root());
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
