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
            if node.get_value() == &value {
                return Some(node);
            }
        }
        None
    }

    pub fn get_mut_by_val(&mut self, value: T) -> Option<&mut TreeNode<T>> {
        if self.get_root().get_value() == &value {
            return Some(self.get_mut_root());
        }
        for node in self.get_mut_root().get_mut_children().iter_mut() {
            if node.get_value().clone() == value {
                return Some(node);
            }
            if !node.get_children().is_empty() {
                return node.get_mut_by_val(value);
            }
        }
        None
    }

    pub fn get_parent_by_val(&self, val: &T) -> &TreeNode<T> {
        for node in self.nodes(self.get_root()) {
            if node.get_children().iter().any(|child| child.get_value() == val) {
                return node;
            }
        }
        panic!("Cannot get node's parent!");
    }

    pub fn get_mut_parent_by_val(&mut self, value: &T) -> &mut TreeNode<T> {
        if self.get_root().get_children().contains(&TreeNode::new(value.clone())) {
            return self.get_mut_root();
        }
        for root_child in self.get_mut_root().get_mut_children().iter_mut() {
            return root_child.get_mut_parent_by_val(value);
        }
        panic!("Cannot get node's parent!");
    }

    pub fn delete_node(&mut self, value: T) {
        self.get_mut_parent_by_val(&value).get_mut_children().retain(|child| {
            child.get_value() != &value
        });
    }

    pub fn get_common_parent_of(&self, value1: &T, value2: &T) -> &TreeNode<T> {
        let first_parent = self.get_parent_by_val(value1);
        let second_parent = self.get_parent_by_val(value2);
        if first_parent == second_parent {
            return first_parent;
        } else {
            return self.get_common_parent_of(first_parent.get_value(), second_parent.get_value());
        }
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
