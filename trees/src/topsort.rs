use std::collections::VecDeque;

fn dfs(
    adjacency_list: &[Vec<usize>],
    visited: &mut [bool],
    at: usize,
    ordering: &mut [usize],
    count: usize,
) -> usize {
    if visited[at] {
        return count;
    }
    visited[at] = true;
    let mut count = count;

    let neighbours = &adjacency_list[at];
    for neighbour in neighbours {
        if !visited[*neighbour] {
            count = dfs(adjacency_list, visited, *neighbour, ordering, count);
        }
    }

    ordering[count] = at;

    if count == 0 {
        return 0; // else overflow
    }
    return count - 1;
}

fn topsort(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let n_nodes = adjacency_list.len();
    let mut visited: Vec<bool> = vec![false; n_nodes];

    let mut ordering: Vec<usize> = vec![0; n_nodes];
    let mut count = n_nodes - 1;

    for i in 0..n_nodes {
        if !visited[i] {
            count = dfs(adjacency_list, &mut visited, i, &mut ordering, count);
        }
    }

    return ordering;
}

fn calculate_in_degree(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let len = adjacency_list.len();
    let mut degree: Vec<usize> = vec![0; len];

    for i in 0..len {
        for to_node in &adjacency_list[i] {
            degree[*to_node] += 1;
        }
    }

    return degree;
}

fn topsort_kahn(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let mut in_degrees = calculate_in_degree(adjacency_list);
    let mut top_queue: VecDeque<usize> = VecDeque::new();
    for i in 0..in_degrees.len() {
        if in_degrees[i] == 0 {
            top_queue.push_back(i);
        }
    }

    let mut ordering: Vec<usize> = Vec::new();
    while let Some(node) = top_queue.pop_front() {
        ordering.push(node);
        for to_node in &adjacency_list[node] {
            in_degrees[*to_node] -= 1;
            if in_degrees[*to_node] == 0 {
                top_queue.push_back(*to_node);
            }
        }
    }

    return ordering;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_topsort_dfs() {
        let graph: Vec<Vec<usize>> = vec![
            vec![3],       // A
            vec![3],       // B
            vec![0, 1],    // C
            vec![6, 7],    // D
            vec![0, 3, 5], // E
            vec![9, 10],   // F
            vec![8],       // G
            vec![8, 9],    // H
            vec![11],      // I
            vec![11, 12],  // J
            vec![9],       // K
            vec![],        // L
            vec![],        // M
        ];
        let ordering = topsort(&graph);
        let expected = vec![4, 5, 10, 2, 1, 0, 3, 7, 9, 12, 6, 8, 11];
        assert_eq!(ordering, expected);
    }

    #[test]
    fn test_topsort_kahn() {
        let graph: Vec<Vec<usize>> = vec![
            vec![3],       // A
            vec![3],       // B
            vec![0, 1],    // C
            vec![6, 7],    // D
            vec![0, 3, 5], // E
            vec![9, 10],   // F
            vec![8],       // G
            vec![8, 9],    // H
            vec![11],      // I
            vec![11, 12],  // J
            vec![9],       // K
            vec![],        // L
            vec![],        // M
        ];
        let ordering: Vec<usize> = topsort_kahn(&graph);
        let expected = vec![2, 4, 1, 0, 5, 3, 10, 6, 7, 8, 9, 11, 12];
        assert_eq!(ordering, expected);
    }
}
