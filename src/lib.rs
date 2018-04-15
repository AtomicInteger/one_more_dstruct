pub mod stack;
pub mod queue;
pub mod list;
pub mod vector_based_dstruct;
pub mod graph;
pub mod graph_node;
pub mod tree;
pub mod tree_node;

#[cfg(test)]
mod tests {
    use vector_based_dstruct::VectorBasedDataStructure;
    use stack::Stack;

    #[test]
    fn push_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn peek_stack() {
        let stack = Stack::from(vec![1, 2, 5]);
        assert_eq!(stack.peek().unwrap().to_owned(), 5);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn clear_stack() {
        let mut not_empty_stack = Stack::from(vec![1, 2, 5]);
        assert_eq!(not_empty_stack.is_empty(), false);
        not_empty_stack.clear();
        assert_eq!(not_empty_stack.is_empty(), true);
    }

    #[test]
    fn is_empty_stack() {
        let not_empty_stack = Stack::from(vec![1, 2, 5]);
        assert_eq!(not_empty_stack.is_empty(), false);
        assert_ne!(not_empty_stack.size(), 0);
        let empty_stack: Stack<i32> = Stack::new();
        assert_eq!(empty_stack.is_empty(), true);
        assert_eq!(empty_stack.size(), 0);
    }

    #[test]
    fn pop_stack() {
        let mut stack = Stack::from(vec![1, 2, 3]);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn push_and_pop_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.size(), 0);
    }

    use queue::Queue;

    #[test]
    fn enqueue_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn dequeue_queue() {
        let mut queue = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn peek_queue() {
        let queue = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.peek().unwrap().to_owned(), 3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn clear_queue() {
        let mut not_empty_queue = Queue::from(vec![1, 2, 5]);
        assert_eq!(not_empty_queue.is_empty(), false);
        not_empty_queue.clear();
        assert_eq!(not_empty_queue.is_empty(), true);
    }

    #[test]
    fn is_empty_queue() {
        let not_empty_queue = Queue::from(vec![1, 2, 5]);
        assert_eq!(not_empty_queue.is_empty(), false);
        assert_ne!(not_empty_queue.size(), 0);
        let empty_queue: Queue<i32> = Queue::new();
        assert_eq!(empty_queue.is_empty(), true);
        assert_eq!(empty_queue.size(), 0);
    }

    use list::List;

    #[test]
    fn push_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn pop_list() {
        let mut list = List::from(vec![1, 2, 3, 4]);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn peek_list() {
        let list = List::from(vec![1, 2, 3, 4]);
        assert_eq!(list.peek().unwrap().to_owned(), 4);
        assert_eq!(list.size(), 4);
    }

    #[test]
    fn unshift_list() {
        let mut list = List::new();
        list.unshift(1);
        list.unshift(2);
        list.unshift(3);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn shift_list() {
        let mut list = List::from(vec![1, 2, 3, 4]);
        assert_eq!(list.shift().unwrap(), 1);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn get_list() {
        let list = List::from(vec![1, 2, 3, 4]);
        assert_eq!(list.get(0).unwrap().to_owned(), 1);
        assert_eq!(list.get(2).unwrap().to_owned(), 3);
        assert_eq!(list.get(3).unwrap().to_owned(), 4);
    }

    use graph::Graph;

    #[test]
    fn add_node_graph() {
        let mut graph = Graph::new(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        assert_eq!(graph.size(), 4);
    }

    #[test]
    fn remove_node_graph() {
        let mut graph = Graph::new(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        assert_eq!(graph.size(), 4);
        graph.remove_node(2);
        assert_eq!(graph.size(), 3);
        assert_eq!(graph.find(2), None);
    }

    #[test]
    fn add_line_graph_node() {
        let mut graph = Graph::new(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.find_mut(1).unwrap().add_line(2);
        assert_eq!(graph.find(1).unwrap().find_by_value(2).unwrap().get_value().clone(), 2);
    }

    use tree::Tree;
    use tree_node::TreeNode;

    #[test]
    fn get_root_tree() {
        let tree = Tree::new(TreeNode::new(0));
        assert_eq!(tree.get_root().get_value(), &0);
        assert_eq!(tree.get_root().clone(), TreeNode::new(0));
        assert_ne!(
            tree.get_root().clone(),
            TreeNode::new_with_children(0, vec![TreeNode::new(0)])
        );
        assert!(tree.get_root().get_children().is_empty());
        assert!(tree.get_children().is_empty());
    }

    #[test]
    fn get_leaves_tree() {
        let tree = Tree::new_with_children(1, vec![TreeNode::new(0), TreeNode::new(2)]);
        assert_eq!(tree.get_leaves(), vec![&TreeNode::new(0), &TreeNode::new(2)]);
        assert_eq!(tree.get_leaves().len(), 2);
        let deep_tree = Tree::new_with_children(
            1,
            vec![
                TreeNode::new_with_children(
                    0,
                    vec![
                        TreeNode::new(10),
                        TreeNode::new_with_children(11, vec![TreeNode::new(12)]),
                    ],
                ),
                TreeNode::new(2),
            ],
        );
        assert_eq!(deep_tree.get_leaves().len(), 3);
        assert!(deep_tree.get_leaves().contains(&&TreeNode::new(2)));
        assert!(deep_tree.get_leaves().contains(&&TreeNode::new(10)));
        assert!(deep_tree.get_leaves().contains(&&TreeNode::new(12)));
        assert!(!deep_tree.get_leaves().contains(&&TreeNode::new(0)));
        assert!(!deep_tree.get_leaves().contains(&&TreeNode::new(11)));
    }

    #[test]
    fn nodes_tree() {
        let tree = Tree::new_with_children(
            1,
            vec![
                TreeNode::new_with_children(0, vec![TreeNode::new(10), TreeNode::new(-1)]),
                TreeNode::new(2),
            ],
        );
        let mut all_nodes = vec![];
        for node in tree.get_children() {
            all_nodes.extend(tree.nodes(&node).iter().cloned());
        }
        assert_eq!(all_nodes.len(), 4);
        assert!(all_nodes.contains(&&TreeNode::new(-1)));
        assert!(all_nodes.contains(&&TreeNode::new(10)));
        assert!(all_nodes.contains(&&TreeNode::new(2)));
        assert!(all_nodes.contains(&&TreeNode::new_with_children(
            0,
            vec![TreeNode::new(10), TreeNode::new(-1)],
        )));
        assert_eq!(tree.nodes(&tree.get_children()[0]).len(), 3);
        assert_eq!(tree.nodes(&tree.get_children()[1]).len(), 1);
        assert_eq!(
            tree.nodes(&tree.get_children()[1].clone()),
            vec![&TreeNode::new(2)]
        );
        assert!(!tree.nodes(&tree.get_children()[0].clone())
            .contains(&&TreeNode::new(2)));
    }

    #[test]
    fn get_by_val_tree() {
        let tree = Tree::new_with_children(
            0,
            vec![
                TreeNode::new_with_children(
                    1,
                    vec![TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)],
                ),
                TreeNode::new(12),
                TreeNode::new_with_children(11, vec![TreeNode::new(76)]),
            ],
        );
        assert_eq!(
            tree.get_by_val(0).unwrap(),
            &TreeNode::new_with_children(
                0,
                vec![
                    TreeNode::new_with_children(
                        1,
                        vec![TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)],
                    ),
                    TreeNode::new(12),
                    TreeNode::new_with_children(11, vec![TreeNode::new(76)]),
                ],
            )
        );
        assert_eq!(tree.get_by_val(12).unwrap(), &TreeNode::new(12));
        assert!(tree.get_by_val(12).unwrap().get_children().is_empty());
        assert_eq!(
            tree.get_by_val(11).unwrap(),
            &TreeNode::new_with_children(11, vec![TreeNode::new(76)])
        );
        assert_eq!(
            tree.get_by_val(11).unwrap().get_children().to_vec(),
            vec![TreeNode::new(76)]
        );
        assert!(tree.get_by_val(3).unwrap().get_children().is_empty());
    }

    #[test]
    fn get_parent_tree() {
        let tree = Tree::new_with_children(
            0,
            vec![
                TreeNode::new_with_children(
                    1,
                    vec![TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)],
                ),
                TreeNode::new(12),
                TreeNode::new_with_children(11, vec![TreeNode::new(76)]),
            ],
        );
        assert_eq!(tree.get_parent_by_val(&1).get_value(), &0);
        assert_eq!(
            tree.get_parent_by_val(&2),
            &TreeNode::new_with_children(
                1,
                vec![TreeNode::new(2), TreeNode::new(3), TreeNode::new(4)],
            )
        );
        assert_eq!(
            tree.get_parent_by_val(&76),
            &TreeNode::new_with_children(11, vec![TreeNode::new(76)])
        );
    }

    #[test]
    #[should_panic]
    fn get_non_existence_parent_tree() {
        let tree = Tree::new_with_children(0, vec![TreeNode::new(1), TreeNode::new(2)]);
        tree.get_parent_by_val(&1000); // Panic: there is no node with value = 1000
    }

    #[test]
    fn add_root_sub_tree() {
        let mut tree = Tree::new(TreeNode::new(0));
        let sub_tree = Tree::new(TreeNode::new(1));
        assert_eq!(tree.get_children().len(), 0);
        tree.add_root_sub_tree(sub_tree);
        assert_eq!(tree.get_children().len(), 1);
        assert_eq!(
            tree.get_children()
                .get(0)
                .unwrap()
                .clone()
                .get_value(),
            &1
        );
        assert_eq!(tree.get_by_val(1).unwrap().get_value(), &1);
        let sub_tree = Tree::new(TreeNode::new(3));
        tree.add_root_sub_tree(sub_tree);
        assert_eq!(tree.get_children().len(), 2);
        assert_eq!(
            tree.get_children()
                .get(1)
                .unwrap()
                .clone()
                .get_value(),
            &3
        );
        assert_eq!(tree.get_by_val(3).unwrap().get_value(), &3);
    }

    #[test]
    fn add_sub_tree() {
        let mut tree = Tree::new_with_children(0, vec![TreeNode::new_with_children(1, vec![TreeNode::new(2)])]);
        let sub_tree = Tree::new(TreeNode::new(10));
        assert_eq!(tree.nodes(tree.get_root()).len(), 4);
        assert!(tree.get_by_val(2).unwrap().get_children().is_empty());
        tree.get_mut_root().add_sub_tree(sub_tree);
        assert_eq!(tree.nodes(tree.get_root()).len(), 5);
        assert!(tree.get_root().get_children().contains(&TreeNode::new(10)));
    }

    #[test]
    fn check_children_reachability_tree_node() {
        let mut tree_node = TreeNode::new_with_children(0, vec![TreeNode::new(1), TreeNode::new(2)]);
        assert_eq!(tree_node.get_children().len(), 2);
        tree_node.get_mut_children().push(TreeNode::new(3));
        assert_eq!(tree_node.get_children().len(), 3);
    }

    #[test]
    fn check_get_by_val_mutability_tree() {
        let mut tree = Tree::new_with_children(0, vec![TreeNode::new_with_children(1, vec![TreeNode::new(3)]), TreeNode::new(2)]);
        assert_eq!(tree.get_by_val(1).unwrap().get_children().len(), 1);
        tree.get_mut_by_val(1).unwrap().get_mut_children().push(TreeNode::new(4));
        assert_eq!(tree.get_by_val(1).unwrap().get_children().len(), 2);
        assert_eq!(tree.get_by_val(0).unwrap().get_children().len(), 2);
        tree.get_mut_by_val(0).unwrap().get_mut_children().clear();
        assert_eq!(tree.get_by_val(0).unwrap().get_children().len(), 0);
    }

    #[test]
    fn get_mut_parent_tree() {
        let mut tree = Tree::new_with_children(0, vec![TreeNode::new(1), TreeNode::new(2)]);
        assert_eq!(tree.get_children().len(), 2);
        tree.get_mut_parent_by_val(&1).get_mut_children().clear();
        assert!(tree.get_children().is_empty())
    }

    #[test]
    fn delete_node_tree() {
        let mut tree = Tree::new_with_children(0, vec![TreeNode::new(1), TreeNode::new(2)]);
        assert_eq!(tree.get_children().len(), 2);
        assert!(tree.get_children().contains(&TreeNode::new(1)));
        assert!(tree.get_children().contains(&TreeNode::new(2)));
        tree.delete_node(2);
        assert_eq!(tree.get_children().len(), 1);
        assert!(tree.get_children().contains(&TreeNode::new(1)));
        assert!(!tree.get_children().contains(&TreeNode::new(2)));
    }

    #[test]
    fn get_common_parent_of_two_nodes_tree() {
        let tree = Tree::new_with_children(0,
                                           vec![TreeNode::new_with_children(1,
                                                                               vec![TreeNode::new(2), TreeNode::new(5)]),
                                                TreeNode::new_with_children(3,
                                                                               vec![TreeNode::new(4), TreeNode::new(6)])]);
        assert_eq!(tree.get_common_parent_of(&1, &3).get_value(), &0);
        assert_eq!(tree.get_common_parent_of(&2, &4).get_value(), &0);
        assert_eq!(tree.get_common_parent_of(&2, &5).get_value(), &1);
        assert_eq!(tree.get_common_parent_of(&4, &6).get_value(), &3);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
