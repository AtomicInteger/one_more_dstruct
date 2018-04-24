extern crate core;

use graph::graph_node::GraphNode;

pub struct Graph<T> {
    node_list: Vec<GraphNode<T>>,
}

impl<T: PartialEq + Clone> Graph<T> {
    pub fn add_node(&mut self, value: T) {
        self.node_list.push(GraphNode::new(value))
    }

    pub fn remove_node(&mut self, value: &T) {
        self.node_list.retain(|node| {
            node.get_value() != value
        });
    }

    pub fn find(&self, value: &T) -> Option<&GraphNode<T>> {
        for node in self.node_list.iter() {
            if node.get_value() == value {
                return Some(node);
            }
        }
        None
    }

    pub fn find_mut(&mut self, value: &T) -> Option<&mut GraphNode<T>> {
        for node in self.node_list.iter_mut() {
            if node.get_value() == value {
                return Some(node);
            }
        }
        None
    }

    pub fn size(&self) -> usize {
        self.node_list.len()
    }

    pub fn new(root_node_value: T) -> Graph<T> {
        Graph {
            node_list: vec![GraphNode::new(root_node_value)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn add_node() {
        let mut graph = Graph::new(0);
        assert_eq!(graph.size(), 1);
        graph.add_node(1);
        assert_eq!(graph.size(), 2);
        graph.add_node(2);
        assert_eq!(graph.size(), 3);
    }

    #[test]
    fn remove_node() {
        let mut graph = Graph::new(0);
        assert_eq!(graph.size(), 1);
        graph.remove_node(&0);
        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn find() {
        let graph = Graph::new(0);
        assert_eq!(graph.find(&0).unwrap().get_value(), &0);
    }

    #[test]
    fn find_mut() {
        let mut graph = Graph::new(0);
        assert!(graph.find(&0).unwrap().get_lines().is_empty());
        // TODO: Fix adding line to non-existent node
        graph.find_mut(&0).unwrap().add_line(1);
        assert_eq!(graph.find(&0).unwrap().get_lines().len(), 1);
        graph.find_mut(&0).unwrap().add_line(2);
        assert_eq!(graph.find(&0).unwrap().get_lines().len(), 2);
    }
}
