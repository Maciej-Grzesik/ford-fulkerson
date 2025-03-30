use petgraph::dot::Dot;
use petgraph::prelude::DiGraphMap;
use plotters::{chart::ChartBuilder, prelude::{BitMapBackend, IntoDrawingArea, IntoSegmentedCoord, PathElement, SegmentValue}, series::LineSeries, style::{IntoFont, BLACK, RED, WHITE}};
use std::{cmp::min, collections::VecDeque, fs::File, io::Write, time::{Duration, Instant}};
use rand::prelude::*;

fn main() {
    //let sizes: Vec<i32> = vec![10, 100, 1000, 10_000, 100_000];
    let sizes: Vec<i32> = vec![10];
    let temperatures: Vec<f64> = vec![0.9]; 
    // let temperatures: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]; 
    let mut results: Vec<(i32, f64, Duration)> = Vec::new();

    for &size in &sizes {
        for &temperature in &temperatures {
            let mut graph = generate_graph(size, temperature, 30);
            let mut nodes = graph.nodes();
            let source = nodes.next().unwrap();
            let sink = nodes.last().unwrap();

            let start = Instant::now();
            let max_flow = ford_fulkerson(&mut graph, source, sink);
            let duration = start.elapsed();

            results.push((size, temperature, duration));
            println!("Graph size: {}, Temperature: {}, Max flow: {}, Time: {:?}", size, temperature, max_flow, duration);
        }
    }

    draw_chart(&results).unwrap();
}

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
                    println!("found path");
                    return true;
                }
            }
        }
    }

    false
}

fn ford_fulkerson(graph: &mut DiGraphMap<i32, i32>, source: i32, sink: i32) -> i32 {
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
    let dot = Dot::with_config(&*graph, &[]);
        let mut file = File::create(format!("graph.dot")).unwrap();
        write!(file, "{:?}", dot).unwrap();

    max_flow
}

fn generate_graph(number_of_nodes: i32, temperature: f64, weight: i32) -> DiGraphMap<i32, i32> {
    let mut rng = rand::thread_rng();
    let mut graph: DiGraphMap<i32, i32> = DiGraphMap::new();

    for i in 0..number_of_nodes {
        graph.add_node(i);
    }

    for i in 0..(number_of_nodes - 1) {
        let w = rng.gen_range(1..weight);
        graph.add_edge(i, i + 1, w);
    }

    let additional_edges = (number_of_nodes as f64 * temperature) as i32;
    for _ in 0..additional_edges {
        let u = rng.gen_range(0..number_of_nodes);
        let v = rng.gen_range(0..number_of_nodes);
        if u != v && !graph.contains_edge(u, v) {
            let w = rng.gen_range(1..weight);
            graph.add_edge(u, v, w);
        }
    }

    graph
}

fn draw_chart(results: &[(i32, f64, Duration)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("results.jpg", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Ford-Fulkerson Algorithm Performance", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (0..results.len()).into_segmented(),
            0..results.iter().map(|(_, _, d)| d.as_millis()).max().unwrap_or(0) as i32,
        )?;

    chart.configure_mesh().draw()?;

    let series: Vec<(SegmentValue<usize>, i32)> = results
        .iter()
        .enumerate()
        .map(|(i, &(_, _, duration))| (SegmentValue::Exact(i), duration.as_millis() as i32))
        .collect();

    chart.draw_series(LineSeries::new(series, &RED))?
        .label("Execution Time")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart.configure_series_labels().background_style(WHITE).border_style(BLACK).draw()?;

    Ok(())
}
