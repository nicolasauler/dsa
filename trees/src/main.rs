#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod basic;

#[derive(Debug)]
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

fn main() {
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
    println!("{:#?}", tree);
}
