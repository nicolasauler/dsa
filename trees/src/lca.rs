use crate::isomorphism::{find_center_tree, root_tree, Tree};

#[derive(Debug)]
struct SparseTable {
    table: Vec<Vec<usize>>,
    indices: Vec<Vec<usize>>,
}

impl SparseTable {
    fn query_min(&self, left: usize, right: usize) -> usize {
        let exp = (1 + right - left).ilog2();
        //let first_r = left + 2_usize.pow(exp) - 1;
        let second_l = right - 2_usize.pow(exp) + 1;

        return self.table[exp as usize][left].min(self.table[exp as usize][second_l]);
    }

    fn query_min_ix(&self, left: usize, right: usize) -> usize {
        let exp = (1 + right - left).ilog2();
        let second_l = right - 2_usize.pow(exp) + 1;

        if self.table[exp as usize][left] <= self.table[exp as usize][second_l] {
            return self.indices[exp as usize][left];
        }
        return self.indices[exp as usize][second_l];
    }
}

fn eulerian_tour(
    tree: &Tree,
    nodes: &mut Vec<usize>,
    depth: &mut Vec<usize>,
    last: &mut [usize],
    node_depth: usize,
) {
    nodes.push(tree.value);
    depth.push(node_depth);
    last[tree.value] = depth.len() - 1;
    for child in &tree.children {
        eulerian_tour(child, nodes, depth, last, node_depth + 1);
        nodes.push(tree.value);
        depth.push(node_depth);
        last[tree.value] = depth.len() - 1;
    }
}

fn make_sparse_table(input: &[usize]) -> SparseTable {
    let col_len = input.len();
    let row_len = col_len.ilog2() as usize + 1;
    let mut indices_table = vec![vec![0; col_len]; row_len];
    let mut sparse_table = vec![vec![0; col_len]; row_len];
    for i in 0..col_len {
        sparse_table[0][i] = input[i];
        indices_table[0][i] = i;
    }

    for i in 1..row_len {
        for j in 0..col_len {
            let k = j + 2_usize.pow(i as u32 - 1);
            if k >= col_len {
                continue;
            }
            let left = sparse_table[i - 1][j];
            let right = sparse_table[i - 1][k];
            sparse_table[i][j] = left.min(right);

            if left <= right {
                indices_table[i][j] = indices_table[i - 1][j];
            } else {
                indices_table[i][j] = indices_table[i - 1][k];
            }
        }
    }

    return SparseTable {
        table: sparse_table,
        indices: indices_table,
    };
}

fn find_lca(adjacency_list: &[Vec<usize>], left_node: usize, right_node: usize) -> usize {
    let center = find_center_tree(adjacency_list);
    let tree = root_tree(
        adjacency_list,
        center[0],
        &mut vec![false; adjacency_list.len()],
    )
    .unwrap();

    let mut nodes = Vec::with_capacity((2 * adjacency_list.len()) - 1);
    let mut depth = Vec::with_capacity((2 * adjacency_list.len()) - 1);
    let mut last = vec![0; adjacency_list.len()];
    eulerian_tour(&tree, &mut nodes, &mut depth, &mut last, 0);

    let sparse_table = make_sparse_table(&depth);

    let left_ix = last[left_node];
    let right_ix = last[right_node];
    let lca_ix = sparse_table.query_min_ix(left_ix.min(right_ix), left_ix.max(right_ix));

    return nodes[lca_ix];
}

fn shortest_path(
    adjacency_list: &[Vec<usize>],
    start: usize,
    end: usize,
    len: usize,
) -> Vec<usize> {
    let mut queue: Vec<usize> = Vec::new();
    queue.push(start);

    let mut visited = vec![false; len];
    visited[start] = true;

    let mut prev = vec![None; len];

    while !queue.is_empty() {
        let node = queue.remove(0);
        let neighbours = &adjacency_list[node];

        for neighbour in neighbours {
            if !visited[*neighbour] {
                queue.push(*neighbour);
                visited[*neighbour] = true;
                prev[*neighbour] = Some(node);
            }
        }
    }

    let mut path = Vec::new();
    let mut goal = end;
    while let Some(next) = prev[goal] {
        path.push(next);
        goal = next;
    }
    path.reverse();

    match path.get(0) {
        Some(&actual_start) => {
            if actual_start != start {
                return Vec::new();
            }
        }
        None => return Vec::new(),
    }
    path.push(end);
    return path;
}

fn find_lca_with_bfs(adjacency_list: &[Vec<usize>], left_node: usize, right_node: usize) -> usize {
    let path_0: Vec<usize> = shortest_path(&adjacency_list, 0, left_node, adjacency_list.len());
    let path_1 = shortest_path(&adjacency_list, 0, right_node, adjacency_list.len());

    let mut lca: usize = 0;
    for i in 0..path_0.len().max(path_1.len()) {
        if let Some(node_0) = path_0.get(i) {
            if let Some(node_1) = path_1.get(i) {
                if node_0 == node_1 {
                    lca = *node_0;
                }
            }
        }
    }

    return lca;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lca0_euler() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca(&tree_adjacency, 6, 5);
        assert_eq!(common_ancestor, 2);
    }

    #[test]
    fn test_find_lca1_euler() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca(&tree_adjacency, 4, 5);
        assert_eq!(common_ancestor, 2);
    }

    #[test]
    fn test_find_lca2_euler() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca(&tree_adjacency, 1, 2);
        assert_eq!(common_ancestor, 0);
    }

    #[test]
    fn test_find_lca3_euler() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca(&tree_adjacency, 3, 2);
        assert_eq!(common_ancestor, 0);
    }

    #[test]
    fn test_find_lca0_bfs() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca_with_bfs(&tree_adjacency, 6, 5);
        assert_eq!(common_ancestor, 2);
    }

    #[test]
    fn test_find_lca1_bfs() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca_with_bfs(&tree_adjacency, 4, 5);
        assert_eq!(common_ancestor, 2);
    }

    #[test]
    fn test_find_lca2_bfs() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca_with_bfs(&tree_adjacency, 1, 2);
        assert_eq!(common_ancestor, 0);
    }

    #[test]
    fn test_find_lca3_bfs() {
        let tree_adjacency = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4, 5],
            vec![1],
            vec![2, 6],
            vec![2],
            vec![4],
        ];
        let common_ancestor = find_lca_with_bfs(&tree_adjacency, 3, 2);
        assert_eq!(common_ancestor, 0);
    }
}
