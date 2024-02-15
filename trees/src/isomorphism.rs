#[derive(Debug, PartialEq)]
struct Tree {
    value: usize,
    children: Vec<Tree>,
}

fn root_tree(adjacency_list: &[Vec<usize>], root: usize, visited: &mut [bool]) -> Option<Tree> {
    if visited[root] {
        return None;
    }
    visited[root] = true;
    let mut tree = Tree {
        value: root,
        children: vec![],
    };

    let neighbours = &adjacency_list[root];
    for neighbour in neighbours {
        if let Some(child_tree) = root_tree(adjacency_list, *neighbour, visited) {
            tree.children.push(child_tree);
        }
    }

    return Some(tree);
}

fn find_center_tree(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let mut leaves = Vec::<usize>::new();
    let mut degrees: Vec<usize> = vec![0; adjacency_list.len()];
    for (i, node) in adjacency_list.iter().enumerate() {
        let n_neighbours = node.len();
        degrees[i] = n_neighbours;
        if n_neighbours < 2 {
            leaves.push(i);
        }
    }

    let mut count = leaves.len();
    while count < adjacency_list.len() {
        let mut new_leaves = Vec::<usize>::new();

        for leaf in &leaves {
            for neighbour in &adjacency_list[*leaf] {
                if degrees[*neighbour] > 0 {
                    degrees[*neighbour] -= 1;
                }
                if degrees[*neighbour] == 1 {
                    new_leaves.push(*neighbour);
                }
            }
        }
        count += new_leaves.len();
        leaves = new_leaves;
    }

    return leaves;
}

fn get_encoding(tree: &Tree) -> String {
    if tree.children.is_empty() {
        return String::from("()");
    }

    let mut encoding = String::from("(");
    let mut child_str: Vec<String> = Vec::new();
    for child in &tree.children {
        child_str.push(get_encoding(child));
    }
    child_str.sort_unstable();
    for child in child_str {
        encoding.push_str(&child);
    }
    encoding.push_str(")");

    return encoding;
}

fn are_trees_isomorphic(tree_0: &[Vec<usize>], tree_1: &[Vec<usize>]) -> bool {
    let center_0 = find_center_tree(tree_0);
    let tree_0 = root_tree(tree_0, center_0[0], &mut vec![false; tree_0.len()]).unwrap();
    let encoding_0 = get_encoding(&tree_0);

    let center_1 = find_center_tree(tree_1);
    for center in center_1 {
        let tree_1 = root_tree(tree_1, center, &mut vec![false; tree_1.len()]).unwrap();
        let encoding_1 = get_encoding(&tree_1);
        if encoding_0 == encoding_1 {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_root_tree() {
        let adjacency_list: Vec<Vec<usize>> = vec![
            vec![2, 1, 5],
            vec![0],
            vec![3, 0],
            vec![2],
            vec![5],
            vec![4, 6, 0],
            vec![5],
        ];
        let mut visited: Vec<bool> = Vec::with_capacity(adjacency_list.len());
        for _ in 0..adjacency_list.len() {
            visited.push(false);
        }

        let tree = root_tree(&adjacency_list, 0, &mut visited);
        let expected = Some(Tree {
            value: 0,
            children: vec![
                Tree {
                    value: 2,
                    children: vec![Tree {
                        value: 3,
                        children: vec![],
                    }],
                },
                Tree {
                    value: 1,
                    children: vec![],
                },
                Tree {
                    value: 5,
                    children: vec![
                        Tree {
                            value: 4,
                            children: vec![],
                        },
                        Tree {
                            value: 6,
                            children: vec![],
                        },
                    ],
                },
            ],
        });
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_find_center_tree() {
        let tree: Vec<Vec<usize>> = vec![
            vec![1],
            vec![0, 3, 4],
            vec![3],
            vec![1, 2, 6, 7],
            vec![1, 5, 8],
            vec![4],
            vec![3, 9],
            vec![3],
            vec![4],
            vec![6],
        ];
        let center = find_center_tree(&tree);
        assert_eq!(center, vec![1, 3]);
    }

    #[test]
    fn test_are_trees_isomorphic() {
        let tree_0: Vec<Vec<usize>> = vec![
            vec![1],
            vec![0, 2, 4],
            vec![1],
            vec![4, 5],
            vec![1, 3],
            vec![3],
        ];
        let tree_1: Vec<Vec<usize>> = vec![
            vec![1],
            vec![0, 2],
            vec![1, 4],
            vec![4],
            vec![2, 3, 5],
            vec![4],
        ];
        let isomorphic = are_trees_isomorphic(&tree_0, &tree_1);
        assert_eq!(isomorphic, true);
    }
}
