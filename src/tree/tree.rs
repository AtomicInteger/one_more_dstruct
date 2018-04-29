use tree::tree_node::TreeNode;

#[derive(Clone, PartialEq, Debug)]
pub struct Tree<T> {
    pub root: TreeNode<T>,
}

impl<T: Clone + PartialEq> Tree<T> {

    pub fn get_leaves(&self) -> Vec<&TreeNode<T>> {
        self.search_leaves(&self.root)
    }

    pub fn nodes(&self) -> Vec<&TreeNode<T>> {
        let mut all_nodes = vec![&self.root];
        for child in self.get_children().iter() {
            all_nodes.extend(child.nodes());
        }
        all_nodes
    }

    pub fn get_children(&self) -> &Vec<TreeNode<T>> {
        self.root.get_children()
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

    pub fn get_by_val(&self, value: &T) -> Option<&TreeNode<T>> {
        for node in self.nodes() {
            if node.get_value() == value {
                return Some(node);
            }
        }
        None
    }

    pub fn get_mut_by_val(&mut self, value: &T) -> Option<&mut TreeNode<T>> {
        if self.root.get_value() == value {
            return Some(&mut self.root);
        }
        for node in self.root.get_mut_children().iter_mut() {
            if node.get_value() == value {
                return Some(node);
            }
            if !node.get_children().is_empty() {
                return node.get_mut_by_val(value);
            }
        }
        None
    }

    pub fn get_parent_by_val(&self, val: &T) -> &TreeNode<T> {
        for node in self.nodes() {
            if node.get_children().iter().any(|child| child.get_value() == val) {
                return node;
            }
        }
        // Checking whether it's root
        if !self.get_by_val(val).is_none() {
            return self.get_by_val(val).unwrap();
        }
        panic!("Cannot get node's parent!");
    }

    pub fn get_mut_parent_by_val(&mut self, value: &T) -> &mut TreeNode<T> {
        if self.root.get_value() == value {
            return &mut self.root;
        }
        if self.get_children().iter().any(|root_child| root_child.get_value() == value) {
            return &mut self.root;
        }
        for child in self.root.get_mut_children().iter_mut() {
            let mut parent = child.get_mut_parent_by_val(value);
            if !parent.is_none() {
                return parent.unwrap();
            }
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
        if first_parent.get_value() == value2 {
            return second_parent;
        } else if second_parent.get_value() == value1 {
            return first_parent;
        }
        if first_parent == second_parent {
            return first_parent;
        } else {
            return self.get_common_parent_of(first_parent.get_value(), second_parent.get_value());
        }
    }

    pub fn add_root_sub_tree(&mut self, sub_tree: Tree<T>) {
        self.root.get_mut_children().push(sub_tree.root);
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

#[cfg(test)]
mod tests {
    use super::Tree;
    use super::TreeNode;

    #[test]
    fn get_leaves() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.get_leaves().len(), 3);
        assert!(tree.get_leaves().contains(&&TreeNode::new(2)));
        assert!(tree.get_leaves().contains(&&TreeNode::new(3)));
        assert!(tree.get_leaves().contains(&&TreeNode::new(5)));
    }

    #[test]
    fn nodes() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.nodes().len(), 6);
        let tree = Tree::new(TreeNode::new(0));
        assert_eq!(tree.nodes().len(), 1);
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(10), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])]),
            TreeNode::new(6), TreeNode::new(7), TreeNode::new(8), TreeNode::new(9)
        ]);
        assert_eq!(tree.nodes().len(), 11);
    }

    #[test]
    fn get_children() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)
        ]);
        assert_eq!(tree.get_children().len(), 4);
        assert_eq!(tree.get_children(), &vec![TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)]);
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2)
        ]);
        assert_eq!(tree.get_children().len(), 2);
        assert_eq!(tree.get_children(), &vec![TreeNode::new(1), TreeNode::new(2)]);
    }

    #[test]
    fn search_leaves() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.search_leaves(&tree.root).len(), 3);
        assert_eq!(tree.search_leaves(&tree.get_by_val(&1).unwrap()).len(), 3);
        assert_eq!(tree.search_leaves(&tree.get_by_val(&4).unwrap()).len(), 1);
    }

    #[test]
    fn get_by_val() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.get_by_val(&0).unwrap().get_children().len(), 1);
        assert_eq!(tree.get_by_val(&1).unwrap().get_children().len(), 3);
        assert_eq!(tree.get_by_val(&4).unwrap().get_children().len(), 1);
        assert_eq!(tree.get_by_val(&5).unwrap().get_children().len(), 0);
    }

    #[test]
    fn get_mut_by_val() {
        let mut tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.get_by_val(&5).unwrap().get_children().len(), 0);
        tree.get_mut_by_val(&5).unwrap().get_mut_children().push(TreeNode::new(6));
        assert_eq!(tree.get_by_val(&5).unwrap().get_children().len(), 1);
        tree.get_mut_by_val(&5).unwrap().get_mut_children().push(TreeNode::new(7));
        tree.get_mut_by_val(&5).unwrap().get_mut_children().push(TreeNode::new(8));
        assert_eq!(tree.get_by_val(&5).unwrap().get_children().len(), 3);
    }

    #[test]
    fn get_parent_by_val() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])])]);
        assert_eq!(tree.get_parent_by_val(&5).get_value(), &4);
        assert_eq!(tree.get_parent_by_val(&4).get_value(), &1);
        assert_eq!(tree.get_parent_by_val(&3).get_value(), &1);
        assert_eq!(tree.get_parent_by_val(&2).get_value(), &1);
        assert_eq!(tree.get_parent_by_val(&1).get_value(), &0);
    }

    #[test]
    fn get_mut_parent_by_val() {
        let mut tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![TreeNode::new(5)])])]);
        assert_eq!(tree.get_by_val(&4).unwrap().get_children().len(), 1);
        tree.get_mut_parent_by_val(&5).get_mut_children().push(TreeNode::new(6));
        assert_eq!(tree.get_by_val(&4).unwrap().get_children().len(), 2);
        assert!(tree.get_by_val(&4).unwrap().get_children().contains(&&TreeNode::new(6)));
        tree.get_mut_parent_by_val(&5).get_mut_children().clear();
        assert_eq!(tree.get_by_val(&4).unwrap().get_children().len(), 0);
    }

    #[test]
    fn delete_node() {
        let mut tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![TreeNode::new(5)])])]);
        assert_eq!(tree.get_by_val(&1).unwrap().get_children().len(), 3);
        assert!(tree.get_by_val(&1).unwrap().get_children().contains(&TreeNode::new(2)));
        tree.delete_node(2);
        assert_eq!(tree.get_by_val(&1).unwrap().get_children().len(), 2);
        assert!(!tree.get_by_val(&1).unwrap().get_children().contains(&TreeNode::new(2)));
    }

    #[test]
    fn get_common_parent_of() {
        let tree = Tree::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                    TreeNode::new(5)])]),
            TreeNode::new_with_children(10, vec![
                TreeNode::new(11)])
        ]);
        assert_eq!(tree.get_parent_by_val(&5).get_value(), &4);
        assert_eq!(tree.get_common_parent_of(&2, &3).get_value(), &1);
        assert_eq!(tree.get_common_parent_of(&5, &11).get_value(), &0);
        assert_eq!(tree.get_common_parent_of(&5, &4).get_value(), &1);
    }

    #[test]
    fn add_root_sub_tree() {
        let mut tree = Tree::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                TreeNode::new(10)
            ])
        ]);
        assert_eq!(tree.get_children().len(), 4);
        assert!(!tree.get_children().contains(&TreeNode::new_with_children(13, vec![TreeNode::new(14)])));
        tree.add_root_sub_tree(Tree::new_with_children(13, vec![TreeNode::new(14)]));
        assert_eq!(tree.get_children().len(), 5);
        assert!(tree.get_children().contains(&TreeNode::new_with_children(13, vec![TreeNode::new(14)])));
    }
}
