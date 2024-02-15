struct Tree {
    value: i32,
    children: Vec<Tree>,
}

// sum of all leaf node values in a tree
fn leaf_node_sum(tree: &Tree) -> i32 {
    if tree.children.is_empty() {
        return tree.value;
    }

    let mut sum = 0;
    for child in &tree.children {
        sum += leaf_node_sum(child);
    }

    return sum;
}

fn tree_height(tree: &Tree) -> usize {
    if tree.children.is_empty() {
        return 0;
    }

    let mut children_height: Vec<usize> = Vec::new();
    for child in &tree.children {
        children_height.push(tree_height(child));
    }
    return children_height.iter().max().unwrap() + 1;
}

#[derive(Debug, PartialEq)]
struct BTree {
    value: usize,
    left: Option<Box<BTree>>,
    right: Option<Box<BTree>>,
}

fn inverting_bin_tree(tree: &mut BTree) {
    if let Some(left) = &mut tree.left {
        inverting_bin_tree(left);
    }
    if let Some(right) = &mut tree.right {
        inverting_bin_tree(right);
    }

    //std::mem::swap(&mut tree.left, &mut tree.right);
    let left = tree.left.take();
    tree.left = tree.right.take();
    tree.right = left;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_node_sum() {
        let tree = Tree {
            value: 5,
            children: vec![
                Tree {
                    value: 4,
                    children: vec![
                        Tree {
                            value: 1,
                            children: vec![
                                Tree {
                                    value: 2,
                                    children: vec![],
                                },
                                Tree {
                                    value: 9,
                                    children: vec![],
                                },
                            ],
                        },
                        Tree {
                            value: -6,
                            children: vec![],
                        },
                    ],
                },
                Tree {
                    value: 3,
                    children: vec![
                        Tree {
                            value: 0,
                            children: vec![],
                        },
                        Tree {
                            value: 7,
                            children: vec![Tree {
                                value: 8,
                                children: vec![],
                            }],
                        },
                        Tree {
                            value: -4,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let sum = leaf_node_sum(&tree);
        assert_eq!(sum, 9);
    }

    #[test]
    fn test_tree_height() {
        let tree = Tree {
            value: 5,
            children: vec![
                Tree {
                    value: 4,
                    children: vec![
                        Tree {
                            value: 1,
                            children: vec![
                                Tree {
                                    value: 2,
                                    children: vec![],
                                },
                                Tree {
                                    value: 9,
                                    children: vec![],
                                },
                            ],
                        },
                        Tree {
                            value: -6,
                            children: vec![],
                        },
                    ],
                },
                Tree {
                    value: 3,
                    children: vec![
                        Tree {
                            value: 0,
                            children: vec![],
                        },
                        Tree {
                            value: 7,
                            children: vec![Tree {
                                value: 8,
                                children: vec![],
                            }],
                        },
                        Tree {
                            value: -4,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let height = tree_height(&tree);
        assert_eq!(height, 3);
    }

    #[test]
    fn test_invert_bin_tree() {
        use super::inverting_bin_tree;
        use super::BTree;

        let mut b_tree = BTree {
            value: 1,
            left: Some(Box::new(BTree {
                value: 2,
                left: Some(Box::new(BTree {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BTree {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(BTree {
                value: 3,
                left: Some(Box::new(BTree {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        inverting_bin_tree(&mut b_tree);
        assert_eq!(
            b_tree,
            BTree {
                value: 1,
                left: Some(Box::new(BTree {
                    value: 3,
                    left: None,
                    right: Some(Box::new(BTree {
                        value: 6,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(BTree {
                    value: 2,
                    left: Some(Box::new(BTree {
                        value: 5,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(BTree {
                        value: 4,
                        left: None,
                        right: None,
                    })),
                })),
            }
        );
    }
}
