extern crate one_more_dstruct as dstruct;

use self::dstruct::tree::tree::Tree;
use self::dstruct::tree::tree_node::TreeNode;

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
    assert_eq!(tree.nodes(&tree.root).len(), 4);
    assert!(tree.get_by_val(2).unwrap().get_children().is_empty());
    tree.root.add_sub_tree(sub_tree);
    assert_eq!(tree.nodes(&tree.root).len(), 5);
    assert!(tree.root.get_children().contains(&TreeNode::new(10)));
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
    assert!(tree.get_children().is_empty());
    let mut tree = Tree::new_with_children(0,
                                           vec![TreeNode::new_with_children(1,
                                                                            vec![TreeNode::new(2), TreeNode::new(5)]),
                                                TreeNode::new_with_children(3,
                                                                            vec![TreeNode::new(4), TreeNode::new(6)])
                                           ],
    );
    assert_eq!(tree.get_by_val(3).unwrap().get_children().len(), 2);
    tree.get_mut_parent_by_val(&6).get_mut_children().clear();
    assert!(tree.get_by_val(3).unwrap().get_children().is_empty())
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
                                                                        vec![TreeNode::new(4), TreeNode::new(6)])
                                       ],
    );
    assert_eq!(tree.get_common_parent_of(&1, &3).get_value(), &0);
    assert_eq!(tree.get_common_parent_of(&2, &4).get_value(), &0);
    assert_eq!(tree.get_common_parent_of(&2, &5).get_value(), &1);
    assert_eq!(tree.get_common_parent_of(&4, &6).get_value(), &3);
}