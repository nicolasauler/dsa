fn depth_first_search(adjacency_list: &[Vec<i32>], at: usize, visited: &mut [bool]) {
    if visited[at] {
        return;
    }
    visited[at] = true;

    let neighbours = &adjacency_list[at];
    for neighbour in neighbours {
        depth_first_search(adjacency_list, *neighbour as usize, visited);
    }
}

fn depth_first_search_ext(
    adjacency_list: &[Vec<i32>],
    at: usize,
    visited: &mut [bool],
    components: &mut [i32],
    count: i32,
) {
    if visited[at] {
        return;
    }
    visited[at] = true;
    components[at] = count;

    let neighbours = &adjacency_list[at];
    for neighbour in neighbours {
        depth_first_search_ext(
            adjacency_list,
            *neighbour as usize,
            visited,
            components,
            count,
        );
    }
}

fn find_components(adjacency_list: &[Vec<i32>]) -> [i32; 18] {
    let mut visited = [false; 18];
    let mut components: [i32; 18] = [0; 18];
    let mut count = 0;

    for i in 0..adjacency_list.len() {
        if !visited[i] {
            depth_first_search_ext(adjacency_list, i, &mut visited, &mut components, count);
            count += 1;
        }
    }

    return components;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_depth_first_search() {
        let adjacency_list: Vec<Vec<i32>> = vec![
            vec![1, 9],       // 0
            vec![0, 8],       // 1
            vec![3],          // 2
            vec![2, 4, 5, 7], // 3
            vec![3],          // 4
            vec![3, 6],       // 5
            vec![5, 7],       // 6
            vec![3, 6, 8, 10, 11],
            vec![1, 7, 9], // 8
            vec![0, 8],    // 9
            vec![7, 11],   // 10
            vec![7, 10],   // 11
            vec![],        // 12
        ];
        let mut visited = [false; 13];

        depth_first_search(&adjacency_list, 0, &mut visited);
        let mut visited_should_be = [true; 13];
        visited_should_be[12] = false;

        assert_eq!(visited, visited_should_be);
    }

    #[test]
    fn test_find_components() {
        let adjacency_list: Vec<Vec<i32>> = vec![
            vec![4, 8, 13, 14], // 0
            vec![5],            // 1
            vec![9, 15],        // 2
            vec![9],            // 3
            vec![0, 8],         // 4
            vec![1, 16, 17],    // 5
            vec![7, 11],        // 6
            vec![6, 11],        // 7
            vec![0, 4, 14],     // 8
            vec![2, 3, 15],     // 9
            vec![15],           // 10
            vec![6, 7],         // 11
            vec![],             // 12
            vec![0, 14],        // 13
            vec![0, 8, 13],     // 14
            vec![2, 9, 10],     // 15
            vec![5],            // 16
            vec![5],            // 17
        ];

        let components = find_components(&adjacency_list);
        let components_should_be = [0, 1, 2, 2, 0, 1, 3, 3, 0, 2, 2, 3, 4, 0, 0, 2, 1, 1];

        assert_eq!(components, components_should_be);
    }
}
