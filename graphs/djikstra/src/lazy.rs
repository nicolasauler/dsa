use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct ProcessedNode(usize, usize);

impl PartialOrd for ProcessedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProcessedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn lazy_djikstra(
    adjacency_list: &[Vec<(usize, usize)>],
    start: usize,
    end: usize,
) -> Vec<Option<usize>> {
    let n_nodes = adjacency_list.len();

    let mut visited: Vec<bool> = vec![false; n_nodes];
    let mut priority_queue: BinaryHeap<Reverse<ProcessedNode>> = BinaryHeap::new();
    priority_queue.push(Reverse(ProcessedNode(start, 0)));
    let mut distances: Vec<usize> = vec![usize::MAX; n_nodes];
    distances[start] = 0;
    let mut prev: Vec<Option<usize>> = vec![None; n_nodes];

    while let Some(Reverse(ProcessedNode(node, min_dist))) = priority_queue.pop() {
        visited[node] = true;
        if distances[node] < min_dist {
            continue;
        }
        for to_node in &adjacency_list[node] {
            if !visited[to_node.0] {
                let dist_through_node = min_dist + to_node.1;
                if distances[to_node.0] > dist_through_node {
                    distances[to_node.0] = dist_through_node;
                    prev[to_node.0] = Some(node);
                    priority_queue.push(Reverse(ProcessedNode(to_node.0, dist_through_node)));
                }
            }
        }
        if node == end {
            break;
        }
    }

    return prev;
}

fn shortest_path(prev: &[Option<usize>], start: usize, end: usize) -> Vec<usize> {
    let mut path: Vec<usize> = Vec::new();
    let mut end = end;

    path.push(end);
    while let Some(node) = prev[end] {
        path.push(node);
        end = node;
    }

    path.reverse();
    if path[0] != start {
        return Vec::new();
    }
    return path;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lazy_djikstra_shortest_path_stop_early() {
        let graph: Vec<Vec<(usize, usize)>> = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![(4, 3)],
            vec![],
        ];
        let prev: Vec<Option<usize>> = lazy_djikstra(&graph, 0, 3);
        let path: Vec<usize> = shortest_path(&prev, 0, 3);
        let expected = vec![0, 2, 1, 3];
        assert_eq!(path, expected);
    }

    #[test]
    fn test_lazy_djikstra_shortest_path() {
        let graph: Vec<Vec<(usize, usize)>> = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![(4, 3)],
            vec![],
        ];
        let prev: Vec<Option<usize>> = lazy_djikstra(&graph, 0, 4);
        let path: Vec<usize> = shortest_path(&prev, 0, 4);
        let expected = vec![0, 2, 1, 3, 4];
        assert_eq!(path, expected);
    }
}
