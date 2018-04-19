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
