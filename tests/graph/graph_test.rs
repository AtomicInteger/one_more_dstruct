extern crate one_more_dstruct as dstruct;

use self::dstruct::graph::graph::Graph;

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