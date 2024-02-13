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
}
