#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    from_node: usize,
    to_node: usize,
    edge_weight: i32,
}

/// Works for graphs with negative weights (djikstra does not)
/// Although it is not as efficient as djikstra
/// Also, edges do not need to be processed in a specific order
/// THIS IS IMPORTANT: this makes it so it can be used in the BGP
/// protocol, for example. The AS will be advertising in whatever order
/// and upon receival, the AS computes using a Bellman Ford-like algorithm.
/// Where instead, for Djikstra, used in OSPF, has to compute in the order
/// of edges that have the minimum weights.
/// So, it would have to recompute from the begining.
fn bellman_ford(edge_list: &[Edge], start: usize, n_nodes: usize) -> Vec<i32> {
    let mut distances: Vec<i32> = vec![i32::MAX; n_nodes];
    distances[start] = 0;

    for _ in 0..(n_nodes - 1) {
        for edge in edge_list {
            let probing_cost = distances[edge.from_node].saturating_add(edge.edge_weight);
            if probing_cost < distances[edge.to_node] {
                distances[edge.to_node] = probing_cost;
            }
        }
    }

    for _ in 0..(n_nodes - 1) {
        for edge in edge_list {
            let potential_cost = distances[edge.from_node] + edge.edge_weight;
            if potential_cost < distances[edge.to_node] {
                distances[edge.to_node] = i32::MIN;
            }
        }
    }

    return distances;
}

fn main() {
    let graph: Vec<Edge> = vec![
        Edge {
            from_node: 2,
            to_node: 0,
            edge_weight: 4,
        },
        Edge {
            from_node: 0,
            to_node: 2,
            edge_weight: 1,
        },
        Edge {
            from_node: 1,
            to_node: 2,
            edge_weight: 6,
        },
        Edge {
            from_node: 0,
            to_node: 1,
            edge_weight: 4,
        },
        Edge {
            from_node: 2,
            to_node: 1,
            edge_weight: 1,
        },
        Edge {
            from_node: 2,
            to_node: 3,
            edge_weight: 2,
        },
    ];
    let distances = bellman_ford(&graph, 0, 4);
    println!("{distances:?}");
}
