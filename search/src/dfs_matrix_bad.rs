fn node_number_from_coord(coord: (usize, usize), col_len: usize) -> usize {
    let (row, col) = coord;
    (row * col_len) + col
}

fn graph_adjacency_list_from_matrix(matrix: &[Vec<bool>]) -> Vec<Vec<usize>> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut graph_adjacency_list: Vec<Vec<usize>> = Vec::with_capacity(row_len * col_len);
    for _ in 0..(row_len * col_len) {
        graph_adjacency_list.push(Vec::new());
    }

    let row_dir: [i32; 4] = [-1, 1, 0, 0];
    let col_dir: [i32; 4] = [0, 0, 1, -1];

    for row in 0..row_len {
        for col in 0..col_len {
            if matrix[row][col] == true {
                for i in 0..4 {
                    let new_row = row as i32 + row_dir[i];
                    let new_col = col as i32 + col_dir[i];
                    if new_row >= 0
                        && new_row < row_len as i32
                        && new_col >= 0
                        && new_col < col_len as i32
                        && matrix[new_row as usize][new_col as usize] == true
                    {
                        graph_adjacency_list[node_number_from_coord((row, col), col_len)].push(
                            node_number_from_coord((new_row as usize, new_col as usize), col_len),
                        );
                    }
                }
            }
        }
    }

    graph_adjacency_list
}

fn dfs(
    graph: &[Vec<usize>],
    components: &mut [Option<usize>],
    at: usize,
    visited: &mut [bool],
    count: usize,
) {
    if visited[at] {
        return;
    }
    visited[at] = true;
    components[at] = Some(count);

    let neighbours = &graph[at];
    for neighbour in neighbours {
        dfs(graph, components, *neighbour, visited, count);
    }
}

fn colour_components(graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let mut components: Vec<Option<usize>> = vec![None; graph.len()];
    let mut visited: Vec<bool> = vec![false; graph.len()];

    let mut count = 0;
    for (i, list) in graph.iter().enumerate() {
        if !list.is_empty() {
            if !visited[i] {
                dfs(graph, &mut components, i, &mut visited, count);
                count += 1;
            }
        }
    }

    components
}

fn remove_islands(matrix: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let adjacency_list = graph_adjacency_list_from_matrix(matrix);
    let connected_components: Vec<Option<usize>> = colour_components(&adjacency_list);

    let mut component_nodes: Vec<Vec<usize>> = Vec::new();
    for (i, component) in connected_components.iter().enumerate() {
        if let Some(comp) = component {
            if let Some(list) = component_nodes.get_mut(*comp) {
                list.push(i)
            } else {
                component_nodes.insert(*comp, vec![i]);
            }
        }
    }

    for i in 0..component_nodes.len() {
        let component_node = &component_nodes[i];
        let mut is_island = true;
        for node in component_node {
            if node % row_len == 0
                || node % col_len == 0
                || node % row_len == row_len - 1
                || node % col_len == col_len - 1
            {
                is_island = false;
                break;
            }
        }

        if is_island {
            component_nodes.remove(i);
        }
    }

    let mut new_matrix = matrix.to_owned();
    for i in 0..row_len {
        for j in 0..col_len {
            let mut erase = true;
            if matrix[i][j]
                && i % row_len != 0
                && j % col_len != 0
                && i % row_len != row_len - 1
                && j % col_len != col_len - 1
            {
                for component in &component_nodes {
                    if component.contains(&node_number_from_coord((i, j), col_len)) {
                        erase = false;
                    }
                }
                if erase {
                    new_matrix[i][j] = false;
                }
            }
        }
    }

    new_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_islands() {
        let matrix: Vec<Vec<bool>> = vec![
            vec![true, false, false, false, false, false],
            vec![false, true, false, true, true, true],
            vec![false, false, true, false, true, false],
            vec![true, true, false, false, true, false],
            vec![true, false, true, true, false, false],
            vec![true, false, false, false, false, true],
        ];

        let new_matrix = remove_islands(&matrix);
        let expected: Vec<Vec<bool>> = vec![
            vec![true, false, false, false, false, false],
            vec![false, false, false, true, true, true],
            vec![false, false, false, false, true, false],
            vec![true, true, false, false, true, false],
            vec![true, false, false, false, false, false],
            vec![true, false, false, false, false, true],
        ];
        assert_eq!(new_matrix, expected);
    }
}
