extern crate core;

use graph_node::GraphNode;

pub struct Graph<T> {
    node_list: Vec<GraphNode<T>>,
}

impl<T: PartialEq + Copy> Graph<T> {
    pub fn add_node(&mut self, value: T) {
        self.node_list.push(GraphNode::new(value))
    }

    pub fn remove_node(&mut self, value: T) {
        for (index, node) in self.node_list.clone().iter().enumerate() {
            if node.value == value {
                self.node_list.remove(index);
            }
        }
    }

    pub fn find(&self, value: T) -> Option<GraphNode<T>> {
        for node in self.node_list.iter() {
            if node.value == value {
                return Some(node.clone());
            }
        }
        None
    }

    fn replace_node(&mut self, value: T, new_node: GraphNode<T>) {
        for (index, node) in self.node_list.clone().iter().enumerate() {
            if node.value == value {
                self.node_list[index] = new_node;
                break;
            }
        }
    }

    pub fn add_line(&mut self, val1: T, val2: T) {
        let mut start_node = self.find(val1).unwrap();
        let end_node = self.find(val2).unwrap();
        start_node.add_line(end_node);
        self.replace_node(start_node.value, start_node);
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
