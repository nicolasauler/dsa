fn solve_bfs(adjacency_list: &[Vec<i32>], start: i32) -> Vec<Option<i32>> {
    let mut queue: Vec<i32> = Vec::new();
    queue.push(start);

    let mut visited: Vec<bool> = Vec::with_capacity(adjacency_list.len());
    for _ in 0..adjacency_list.len() {
        visited.push(false);
    }
    visited[start as usize] = true;
    let mut prev: Vec<Option<i32>> = Vec::with_capacity(adjacency_list.len());
    for _ in 0..adjacency_list.len() {
        prev.push(None);
    }

    while !queue.is_empty() {
        let node = queue.remove(0);
        let neighbours = &adjacency_list[node as usize];

        for neighbour in neighbours {
            if !visited[*neighbour as usize] {
                queue.push(*neighbour);
                visited[*neighbour as usize] = true;
                prev[*neighbour as usize] = Some(node);
            }
        }
    }

    return prev;
}

fn shortest_path(prev: &[Option<i32>], start: i32, end: i32) -> Vec<i32> {
    let mut path = Vec::new();
    let mut end = end;

    while let Some(parent) = prev[end as usize] {
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

pub fn bfs(adjacency_list: &[Vec<i32>], start: i32, end: i32) -> Vec<i32> {
    let prev = solve_bfs(adjacency_list, start);
    return shortest_path(&prev, start, end);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bfs() {
        let adjacency_list: Vec<Vec<i32>> = vec![
            vec![9, 7, 11],    // 0
            vec![8, 10],       // 1
            vec![3, 12],       // 2
            vec![2, 4, 7],     // 3
            vec![3],           // 4
            vec![6],           // 5
            vec![5, 7],        // 6
            vec![0, 3, 6, 11], // 7
            vec![1, 12, 9],    // 8
            vec![0, 8, 10],    // 9
            vec![1, 9],        // 10
            vec![0, 7],        // 11
            vec![2, 8],        // 12
        ];

        let result = bfs(&adjacency_list, 0, 1);
        assert_eq!(result, vec![0, 9, 8]);
    }
}
