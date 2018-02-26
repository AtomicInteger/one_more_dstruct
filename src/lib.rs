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
    fn add_lines_graph() {
        let mut graph = Graph::new(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_line(1, 2);
        assert_eq!(graph.find(1).unwrap().find_by_value(2).unwrap().value, 2);
    }

    use tree::Tree;
    use tree_node::TreeNode;

    #[test]
    fn get_root_tree() {
        let tree = Tree::new(TreeNode::new(0));
        assert_eq!(tree.get_root().get_value().unwrap(), 0);
    }

    #[test]
    fn get_leaves_tree() {
        let tree = Tree::new_with_children(1, vec![Some(TreeNode::new(0)), Some(TreeNode::new(2))]);
        assert_eq!(tree.get_leaves(), vec![TreeNode::new(0), TreeNode::new(2)]);
    }

    #[test]
    fn nodes_tree() {
        let tree = Tree::new_with_children(1, vec![
                Some(TreeNode::new_with_children(0, vec![
                        Some(TreeNode::new(10)),
                        Some(TreeNode::new(-1))
                    ])),
                Some(TreeNode::new(2))
        ]);
        let mut all_nodes = vec![];
        for node in tree.get_children() {
            all_nodes.extend(tree.nodes(node.unwrap()).iter().cloned());
        }
        assert_eq!(all_nodes.len(), 4);
        assert!(all_nodes.contains(&TreeNode::new(-1)));
        assert!(all_nodes.contains(&TreeNode::new(10)));
        assert!(all_nodes.contains(&TreeNode::new(2)));
        assert!(all_nodes.contains(&TreeNode::new(0)));
        assert_eq!(tree.nodes(tree.get_children()[0].clone().unwrap()).len(), 3);
        assert!(!tree.nodes(tree.get_children()[0].clone().unwrap()).contains(&TreeNode::new(2)));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
