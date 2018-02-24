pub mod stack;
pub mod queue;
pub mod list;
pub mod vector_based_dstruct;
pub mod graph;
pub mod graph_node;
pub mod tree;

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
        let stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(stack.peek().unwrap().to_owned(), 5);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn clear_stack() {
        let mut not_empty_stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(not_empty_stack.is_empty(), false);
        not_empty_stack.clear();
        assert_eq!(not_empty_stack.is_empty(), true);
    }

    #[test]
    fn is_empty_stack() {
        let not_empty_stack = Stack::from(vec!(1, 2, 5));
        assert_eq!(not_empty_stack.is_empty(), false);
        assert_ne!(not_empty_stack.size(), 0);
        let empty_stack : Stack<i32> = Stack::new();
        assert_eq!(empty_stack.is_empty(), true);
        assert_eq!(empty_stack.size(), 0);
    }

    #[test]
    fn pop_stack() {
        let mut stack = Stack::from(vec!(1, 2, 3));
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
        let mut queue = Queue::from(vec!(1, 2, 3));
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn peek_queue() {
        let queue = Queue::from(vec!(1, 2, 3));
        assert_eq!(queue.peek().unwrap().to_owned(), 3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn clear_queue() {
        let mut not_empty_queue = Queue::from(vec!(1, 2, 5));
        assert_eq!(not_empty_queue.is_empty(), false);
        not_empty_queue.clear();
        assert_eq!(not_empty_queue.is_empty(), true);
    }

    #[test]
    fn is_empty_queue() {
        let not_empty_queue = Queue::from(vec!(1, 2, 5));
        assert_eq!(not_empty_queue.is_empty(), false);
        assert_ne!(not_empty_queue.size(), 0);
        let empty_queue : Queue<i32> = Queue::new();
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
        let mut list = List::from(vec!(1, 2, 3, 4));
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn peek_list() {
        let list = List::from(vec!(1, 2, 3, 4));
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
        let mut list = List::from(vec!(1, 2, 3, 4));
        assert_eq!(list.shift().unwrap(), 1);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn get_list() {
        let list = List::from(vec!(1, 2, 3, 4));
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
    use graph_node::GraphNode;
    #[test]
    fn get_root_tree() {
        let tree = Tree::new(GraphNode::new(0));
        assert_eq!(tree.get_root().unwrap().value, 0);
    }

    #[test]
    fn get_leaf_tree() {
        let tree = Tree::new_with_children(GraphNode::new(1), (Some(GraphNode::new(0)), Some(GraphNode::new(2))));
        assert_eq!(tree.get_leaf(), vec!(GraphNode::new(0), GraphNode::new(2)));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
