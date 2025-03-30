use petgraph::prelude::DiGraphMap;
use std::{cmp::min, collections::VecDeque};

fn bfs(graph: &DiGraphMap<i32, i32>, source: i32, sink: i32, parent: &mut Vec<Option<i32>>) -> bool {
    let mut visited: Vec<bool> = vec![false; graph.node_count()];
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(source);
    visited[source as usize] = true;
    parent[source as usize] = None;

    while !queue.is_empty() {
        let current_node: i32 = queue.pop_front().unwrap();
        for neighbor in graph.neighbors(current_node) {
            if !visited[neighbor as usize] && *graph.edge_weight(current_node, neighbor).unwrap_or(&0) > 0 {
                queue.push_back(neighbor);
                parent[neighbor as usize] = Some(current_node);
                visited[neighbor as usize] = true;

                if neighbor == sink {
                    return true;
                }
            }
        }
    }

    false
}

pub fn ford_fulkerson(graph: &mut DiGraphMap<i32, i32>, source: i32, sink: i32) -> i32 {
    let mut parent: Vec<Option<i32>> = vec![None; graph.node_count()];
    let mut max_flow = 0;

    while bfs(graph, source, sink, &mut parent) {
        let mut path_flow = i32::MAX;
        let mut current_node = sink;

        while let Some(prev_node) = parent[current_node as usize] {
            path_flow = min(path_flow, *graph.edge_weight(prev_node, current_node).unwrap());
            current_node = prev_node;
        }

        let mut node_on_path = sink;
        while let Some(previous_node) = parent[node_on_path as usize] {
            let weight = graph.edge_weight_mut(previous_node, node_on_path).unwrap();
            *weight -= path_flow;
            if let Some(reverse_weight) = graph.edge_weight_mut(node_on_path, previous_node) {
                *reverse_weight += path_flow;
            } else {
                graph.add_edge(node_on_path, previous_node, path_flow);
            }
            node_on_path = previous_node;
        }

        max_flow += path_flow;

        
    }
    //let dot = Dot::with_config(&*graph, &[]);
    //    let mut file = File::create(format!("graph.dot")).unwrap();
    //    write!(file, "{:?}", dot).unwrap();

    max_flow
}
