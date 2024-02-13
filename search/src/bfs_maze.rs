use crate::bfs::bfs;

fn graph_from_maze(maze: &[[bool; 7]; 5]) -> Vec<Vec<i32>> {
    let mut graph: Vec<Vec<i32>> = vec![];

    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            let mut neighbours: Vec<i32> = vec![];

            if i > 0 && maze[i - 1][j] {
                neighbours.push((i - 1) as i32 * maze[i].len() as i32 + j as i32);
            }
            if i < maze.len() - 1 && maze[i + 1][j] {
                neighbours.push((i + 1) as i32 * maze[i].len() as i32 + j as i32);
            }
            if j > 0 && maze[i][j - 1] {
                neighbours.push(i as i32 * maze[i].len() as i32 + (j - 1) as i32);
            }
            if j < maze[i].len() - 1 && maze[i][j + 1] {
                neighbours.push(i as i32 * maze[i].len() as i32 + (j + 1) as i32);
            }

            graph.push(neighbours);
        }
    }

    return graph;
}

fn maze_coords_from_graph_path(
    shortest_path: Vec<i32>,
    maze: &[[bool; 7]; 5],
) -> Vec<(usize, usize)> {
    shortest_path
        .iter()
        .map(|&node| {
            let row = (node / maze[0].len() as i32) as usize;
            let col = (node % maze[0].len() as i32) as usize;
            (row, col)
        })
        .collect()
}

fn solve_maze_by_graph(maze: &[[bool; 7]; 5]) -> Vec<(usize, usize)> {
    let adjacency_list = graph_from_maze(&maze);
    println!("adjacency_list: {:?}", adjacency_list);
    let shortest_path = bfs(&adjacency_list, 0, 31);
    return maze_coords_from_graph_path(shortest_path, &maze);
}

fn solve_maze(
    maze: &[[bool; 7]; 5],
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<Option<(usize, usize)>>> {
    let maze_len = maze.len();

    let mut row_queue: Vec<usize> = Vec::new();
    let mut col_queue: Vec<usize> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(maze_len);
    for i in 0..maze_len {
        visited.push(Vec::with_capacity(maze[0].len()));
        for _ in 0..maze[0].len() {
            visited[i].push(false);
        }
    }
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = Vec::with_capacity(maze_len);
    for i in 0..maze_len {
        prev.push(Vec::with_capacity(maze[0].len()));
        for _ in 0..maze[0].len() {
            prev[i].push(None);
        }
    }

    row_queue.push(start.0);
    col_queue.push(start.1);
    visited[start.0][start.1] = true;

    while !row_queue.is_empty() && !col_queue.is_empty() {
        let row = row_queue.remove(0);
        let col = col_queue.remove(0);

        if (row, col) == end {
            break;
        }

        let row_dir: [i32; 4] = [-1, 1, 0, 0];
        let col_dir = [0, 0, 1, -1];
        for i in 0..4 {
            let new_row = (row as i32) + row_dir[i];
            let new_col = (col as i32) + col_dir[i];
            if new_row >= 0
                && new_row < (maze.len() as i32)
                && new_col >= 0
                && new_col < (maze[0].len() as i32)
                && !visited[new_row as usize][new_col as usize]
                && maze[new_row as usize][new_col as usize]
            {
                row_queue.push(new_row.try_into().unwrap());
                col_queue.push(new_col.try_into().unwrap());
                visited[new_row as usize][new_col as usize] = true;
                prev[new_row as usize][new_col as usize] = Some((row, col));
            }
        }
    }

    return prev;
}

fn shortest_path_from_maze(
    prev: &[Vec<Option<(usize, usize)>>],
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut end = end;

    while let Some(parent) = prev[end.0][end.1] {
        path.push(parent);
        end = parent;
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
    return path;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph_from_maze() {
        let maze: [[bool; 7]; 5] = [
            [true, true, true, false, true, true, true],
            [true, false, true, true, true, false, true],
            [true, false, true, true, true, true, true],
            [true, true, false, false, true, true, true],
            [false, true, false, true, true, false, true],
        ];
        let graph = graph_from_maze(&maze);
        let expected = vec![
            vec![7, 1],
            vec![0, 2],
            vec![9, 1],
            vec![10, 2, 4],
            vec![11, 5],
            vec![4, 6],
            vec![13, 5],
            vec![0, 14],
            vec![1, 7, 9],
            vec![2, 16, 10],
            vec![17, 9, 11],
            vec![4, 18, 10],
            vec![5, 19, 11, 13],
            vec![6, 20],
            vec![7, 21],
            vec![22, 14, 16],
            vec![9, 17],
            vec![10, 16, 18],
            vec![11, 25, 17, 19],
            vec![26, 18, 20],
            vec![13, 27, 19],
            vec![14, 22],
            vec![29, 21],
            vec![16, 22],
            vec![17, 31, 25],
            vec![18, 32, 26],
            vec![19, 25, 27],
            vec![20, 34, 26],
            vec![21, 29],
            vec![22],
            vec![29, 31],
            vec![32],
            vec![25, 31],
            vec![26, 32, 34],
            vec![27],
        ];
        assert_eq!(graph, expected);
    }

    #[test]
    fn test_solve_maze_by_graph() {
        let maze: [[bool; 7]; 5] = [
            [true, true, true, false, true, true, true],
            [true, false, true, true, true, false, true],
            [true, false, true, true, true, true, true],
            [true, true, false, false, true, true, true],
            [false, true, false, true, true, false, true],
        ];
        let solution = solve_maze_by_graph(&maze);
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 4),
        ];
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_shortest_path_from_maze() {
        let maze: [[bool; 7]; 5] = [
            [true, true, true, false, true, true, true],
            [true, false, true, true, true, false, true],
            [true, false, true, true, true, true, true],
            [true, true, false, false, true, true, true],
            [false, true, false, true, true, false, true],
        ];
        let prev = solve_maze(&maze, (0, 0), (4, 3));
        let path = shortest_path_from_maze(&prev, (0, 0), (4, 3));
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 4),
        ];
        assert_eq!(path, expected);
    }
}
