#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod basic;
mod isomorphism;
mod lca;
mod topsort;

use topsort::topsort_kahn_weighted;

fn construct_path_from_prev(
    prev: &[Option<usize>],
    start: usize,
    end: usize,
) -> Result<Vec<usize>, ()> {
    let mut path = Vec::new();
    let mut end = end;

    path.push(end);
    while let Some(node) = prev[end] {
        path.push(node);
        end = node;
    }
    path.reverse();

    if path[0] != start {
        return Err(());
    }
    return Ok(path);
}

fn shortest_path_topsort(
    adjacency_list: &[Vec<(usize, i32)>],
    start: usize,
    end: usize,
) -> Vec<usize> {
    let n_nodes = adjacency_list.len();
    let ordering = topsort_kahn_weighted(adjacency_list);

    let mut distances: Vec<i32> = vec![i32::MAX; n_nodes];
    distances[start] = 0;
    let mut prev: Vec<Option<usize>> = vec![None; n_nodes];

    for node in ordering {
        for (to_node, weight) in &adjacency_list[node] {
            let curr_dist = distances[*to_node];
            if distances[node] == i32::MAX {
                continue;
            }
            if curr_dist > distances[node] + weight {
                prev[*to_node] = Some(node);
                distances[*to_node] = distances[node] + weight;
            }
        }
    }

    return construct_path_from_prev(&prev, start, end).unwrap();
}

fn main() {
    let graph: Vec<Vec<(usize, i32)>> = vec![
        vec![(1, 3), (2, 6)],          // A (0)
        vec![(2, 4), (3, 4), (5, 11)], // B (1)
        vec![(3, 8), (6, 11)],         // C (2)
        vec![(4, -4), (5, 5), (6, 2)],
        vec![(7, 9)],
        vec![(7, 1)],
        vec![(7, 2)],
        vec![],
    ];
    let path = shortest_path_topsort(&graph, 3, 7);
    for node in path {
        print!("{} -> ", (65 + node) as u8 as char);
    }
}
