#[derive(Debug, Clone)]
pub struct GraphNode<V> {
    pub value : V,
    pub lines : Vec<GraphNode<V>>
}

impl<V : Copy + PartialEq> GraphNode<V> {
    pub fn new(value : V) -> GraphNode<V> {
        GraphNode{value, lines: Vec::new()}
    }

    pub fn add_line(&mut self, node : GraphNode<V>) {
        self.lines.push(node);
    }

    pub fn find_by_value(&mut self, value : V) -> Option<GraphNode<V>>{
        for node in self.lines.iter() {
            if node.value == value {
                return Some(node.clone())
            }
        }
        None
    }
}

impl<V : PartialEq> PartialEq for GraphNode<V> {
    fn eq(&self, other: &GraphNode<V>) -> bool {
        self.value == other.value
    }
}

