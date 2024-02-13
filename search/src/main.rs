#![allow(dead_code)]

use crate::bfs::bfs;
mod bfs;
mod binary_search;
mod dfs;

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

fn solve_maze_by_graph(maze: [[bool; 7]; 5]) {
    let adjacency_list = graph_from_maze(&maze);
    let shortest_path = bfs(&adjacency_list, 0, 31);
    let maze_solution = maze_coords_from_graph_path(shortest_path, &maze);
    println!("maze_solution: {:?}", maze_solution);
}

fn main() {
    let maze: [[bool; 7]; 5] = [
        [true, true, true, false, true, true, true],
        [true, false, true, true, true, false, true],
        [true, false, true, true, true, true, true],
        [true, true, false, false, true, true, true],
        [false, true, false, true, true, false, true],
    ];
    solve_maze_by_graph(maze);
}
