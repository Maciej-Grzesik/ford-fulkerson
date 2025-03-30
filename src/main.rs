use plotters::prelude::*;
use rand::{prelude::*, random_range};
use std::time::{Duration, Instant};
use petgraph::prelude::DiGraphMap;
use statrs::statistics::Statistics;

mod algorithm;
use algorithm::ford_fulkerson::ford_fulkerson;
mod core;
use core::logspace::logspace;

macro_rules! measure_time {
    ($fn_call:expr) => {{
        let start = Instant::now();
        $fn_call;
        let elapsed = start.elapsed().as_secs_f64();

        elapsed
    }};
}

fn main() {
    //let sizes: Vec<i32> = vec![10, 100, 1000, 10_000, 100_000, 1_000_000];
    let sizes = logspace(100, 1_000_000, 10);
    let max_duration = Duration::from_secs(300);
    let max_iterations = 1_000;

    let mut results = Vec::new();

    for &size in &sizes {
        let mut times = Vec::new();
        let start_experiment = Instant::now();
        let mut iteration = 0;

        while start_experiment.elapsed() < max_duration && iteration < max_iterations {
            let mut graph = generate_graph(size, 1.0, 30);
            let mut nodes = graph.nodes();
            let source = nodes.next().unwrap();
            let sink = nodes.last().unwrap();

            let elapsed = measure_time!(ford_fulkerson(&mut graph, source, sink));

            times.push(elapsed);

            iteration += 1;
        }

        if !times.is_empty() {
            let mean = times.clone().mean();
            let std_dev = times.std_dev();
            results.push((size as f64, mean, std_dev));
            println!("Size: {}, Mean Time: {:.4} s, Std Dev: {:.4}", size, mean, std_dev);
        } else {
            print!("Size: {}, not enoudh time for measurement", size);
        }
    }

    plot_results(&results);
}


fn generate_graph(number_of_nodes: i32, temperature: f64, weight: i32) -> DiGraphMap<i32, i32> {
    let mut rng = rand::rng();
    let mut graph: DiGraphMap<i32, i32> = DiGraphMap::new();

    for i in 0..number_of_nodes {
        graph.add_node(i);
    }

    for i in 0..(number_of_nodes - 1) {
        let w = random_range(1..weight);
        graph.add_edge(i, i + 1, w);
    }

    let additional_edges = (number_of_nodes as f64 * temperature) as i32;
    for _ in 0..additional_edges {
        let u = random_range(0..number_of_nodes);
        let v = random_range(0..number_of_nodes);
        if u != v && !graph.contains_edge(u, v) {
            let w = rng.random_range(1..weight);
            graph.add_edge(u, v, w);
        }
    }

    graph
}

fn plot_results(data: &Vec<(f64, f64, f64)>) {
    let root = BitMapBackend::new("results.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    //let min_x = data.first().unwrap().0;
    let max_x = data.last().unwrap().0;
    let max_y = data.iter().map(|(_, mean, err)| mean+err).fold(f64::NEG_INFINITY, f64::max);
    //let min_y = data.iter().map(|(_, mean, err)| mean-err).fold(f64::INFINITY, f64::min);

    let mut chart = ChartBuilder::on(&root)
        .caption("Ford-Fulkerson execution time", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (0.0..max_x+1.0).log_scale(),
            0.0..max_y+1.0,
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Graph size (log scale)")
        .y_desc("Mean execution time")
        .draw()
        .unwrap();

    let points: Vec<(f64,f64)> = data.iter().map(|&(x,y,_)| (x,y)).collect();

    chart.draw_series(LineSeries::new(points, BLUE))
        .unwrap()
        .label("Mean execution time")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x+10, y)], BLUE));

    chart
        .draw_series(data.iter().map(|&(x,y,err)| {
            ErrorBar::new_vertical(x, y-err, y, y+err, BLUE.filled(), 10)
        }))
        .unwrap();
    
    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}


