#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode<V> {
    value: V,
    lines: Vec<GraphNode<V>>,
}

impl<V: PartialEq + Clone> GraphNode<V> {
    pub fn get_value(&self) -> &V {
        &self.value
    }

    pub fn get_mut_value(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn get_lines(&self) -> &Vec<GraphNode<V>> {
        &self.lines
    }

    pub fn get_mut_lines(&mut self) -> &mut Vec<GraphNode<V>> {
        &mut self.lines
    }

    pub fn new(value: V) -> GraphNode<V> {
        GraphNode {
            value,
            lines: Vec::new(),
        }
    }

    pub fn add_line(&mut self, node_value: V) {
        self.lines.push(GraphNode::new(node_value));
    }

    pub fn find_by_value(&self, value: V) -> Option<&GraphNode<V>> {
        for node in self.lines.iter() {
            if node.value == value {
                return Some(node);
            }
        }
        None
    }
}
