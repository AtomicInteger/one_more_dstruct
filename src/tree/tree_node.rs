use tree::tree::Tree;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T: PartialEq + Clone> TreeNode<T> {
    pub fn get_value(&self) -> &T {
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
            value,
            children: vec![],
        }
    }

    pub fn new_with_children(value: T, children: Vec<TreeNode<T>>) -> TreeNode<T> {
        TreeNode {
            value,
            children,
        }
    }

    pub fn add_sub_tree(&mut self, sub_tree: Tree<T>) {
        self.get_mut_children().push(sub_tree.root);
    }

    pub fn get_mut_by_val(&mut self, value: &T) -> Option<&mut TreeNode<T>> {
        for node in self.get_mut_children().iter_mut() {
            if node.get_value() == value {
                return Some(node);
            }
            if !node.get_children().is_empty() {
                return node.get_mut_by_val(value);
            }
        }
        None
    }

    pub fn get_mut_parent_by_val(&mut self, value: &T) -> Option<&mut TreeNode<T>> {
        if self.get_children().iter().any(|child| child.get_value() == value) {
            return Some(self);
        }
        for child in self.get_mut_children().iter_mut() {
            if !child.get_children().is_empty() {
                return child.get_mut_parent_by_val(value);
            }
        }
        None
    }

    pub fn nodes(&self) -> Vec<&TreeNode<T>> {
        let mut all_nodes = vec![self];
        for child in self.get_children().iter() {
            all_nodes.extend(child.nodes());
        }
        all_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::{TreeNode, Tree};

    #[test]
    fn get_value() {
        let tree_node = TreeNode::new_with_children(0, vec![TreeNode::new(2)]);
        assert_eq!(tree_node.get_value(), &0);
    }

    #[test]
    fn get_children() {
        let tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                TreeNode::new(5)
            ])
        ]);
        assert_eq!(tree_node.get_children().len(), 4);
        assert!(tree_node.get_children().contains(&TreeNode::new(1)));
        assert!(tree_node.get_children().contains(&TreeNode::new(2)));
        assert!(tree_node.get_children().contains(&TreeNode::new(3)));
        assert!(tree_node.get_children().contains(&TreeNode::new_with_children(4, vec![
            TreeNode::new(5)
        ])));
    }

    #[test]
    fn get_mut_children() {
        let mut tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                TreeNode::new(5)
            ])
        ]);
        assert_eq!(tree_node.get_children().len(), 4);
        assert!(!tree_node.get_children().contains(&TreeNode::new(10)));
        tree_node.get_mut_children().push(TreeNode::new(10));
        assert_eq!(tree_node.get_children().len(), 5);
        assert!(tree_node.get_children().contains(&TreeNode::new(10)));
    }

    #[test]
    fn add_sub_tree() {
        let mut tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                TreeNode::new(5)
            ])
        ]);
        let sub_tree = Tree::new_with_children(10, vec![
            TreeNode::new(11), TreeNode::new(12), TreeNode::new(13)
        ]);
        assert_eq!(tree_node.get_children().len(), 4);
        assert!(!tree_node.get_children().contains(&TreeNode::new_with_children(10, vec![
            TreeNode::new(11), TreeNode::new(12), TreeNode::new(13)
        ])));
        tree_node.add_sub_tree(sub_tree);
        assert_eq!(tree_node.get_children().len(), 5);
        assert!(tree_node.get_children().contains(&TreeNode::new_with_children(10, vec![
            TreeNode::new(11), TreeNode::new(12), TreeNode::new(13)
        ])));
    }

    #[test]
    fn get_mut_by_val() {
        let mut tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3), TreeNode::new_with_children(4, vec![
                TreeNode::new(5)
            ])
        ]);
        assert_eq!(tree_node.nodes().len(), 6);
        assert!(!tree_node.nodes().contains(&&TreeNode::new(10)));
        tree_node.get_mut_by_val(&1).unwrap().get_mut_children().push(TreeNode::new(10));
        assert_eq!(tree_node.nodes().len(), 7);
        assert!(tree_node.nodes().contains(&&TreeNode::new(10)));
    }

    #[test]
    fn get_mut_parent_by_val() {
        let mut tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new(1), TreeNode::new(2), TreeNode::new(3)
        ]);
        assert_eq!(tree_node.get_children().len(), 3);
        assert!(!tree_node.get_children().contains(&TreeNode::new(4)));
        tree_node.get_mut_parent_by_val(&2).unwrap().get_mut_children().push(TreeNode::new(4));
        assert_eq!(tree_node.get_children().len(), 4);
        assert!(tree_node.get_children().contains(&TreeNode::new(4)));
    }

    #[test]
    fn nodes() {
        let tree_node = TreeNode::new_with_children(0, vec![
            TreeNode::new_with_children(1, vec![
                TreeNode::new(2), TreeNode::new(3)
            ]),
            TreeNode::new_with_children(4, vec![
                TreeNode::new(5), TreeNode::new(6), TreeNode::new(7), TreeNode::new(8)
            ]),
            TreeNode::new(9)
        ]);
        assert_eq!(tree_node.nodes().len(), 10);
        assert!(tree_node.nodes().contains(&&TreeNode::new_with_children(1, vec![
            TreeNode::new(2), TreeNode::new(3)
        ])));
        assert!(tree_node.nodes().contains(&&TreeNode::new_with_children(4, vec![
            TreeNode::new(5), TreeNode::new(6), TreeNode::new(7), TreeNode::new(8)
        ])));
        assert!(tree_node.nodes().contains(&&TreeNode::new(9)));
    }
}