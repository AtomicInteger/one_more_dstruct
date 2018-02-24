use graph_node::GraphNode;

pub struct Tree<T> {
    root : Option<GraphNode<T>>,
    children : (Option<GraphNode<T>>, Option<GraphNode<T>>)
}

impl<T : Clone> Tree<T> {
    pub fn get_root(&self) -> Option<GraphNode<T>> {
        self.root.clone()
    }

    pub fn get_leaf(&self) -> Vec<GraphNode<T>> {
        let mut left_leaf = self.search_leaf(&self.children.0.clone().unwrap());
        let right_leaf = self.search_leaf(&self.children.1.clone().unwrap());
        left_leaf.extend(right_leaf.iter().cloned());
        left_leaf
    }

    fn search_leaf(&self, node : &GraphNode<T>) -> Vec<GraphNode<T>> {
        let mut result = vec!();
        if node.lines.is_empty() {
            return vec!(node.to_owned());
        }
        node.lines.iter().for_each(|child| {
            if child.lines.is_empty() {
                result.push(child.clone());
            } else {
                let inner_leaf = self.search_leaf(child);
                result.extend(inner_leaf.iter().cloned());
            }
        });
        result
    }

    pub fn new(root : GraphNode<T>) -> Tree<T> {
        Tree{ root : Some(root), children : (None, None)}
    }

    pub fn new_with_children(root : GraphNode<T>, children : (Option<GraphNode<T>>, Option<GraphNode<T>>)) -> Tree<T> {
        Tree{ root : Some(root), children}
    }
}