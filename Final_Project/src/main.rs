mod plotter;
mod graph; 
mod find_hubs;
use graph::Graph;
use find_hubs::find_hubs;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use ndarray::{Array1};


// feed the distribution input to estimate the parameters of a power-law distribution directly from the data without employing a full-fledged linear regression model.
fn fit_power_law(distribution: &HashMap<u32, u32>) -> (f64, f64) {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for (length, count) in distribution {
        if *length > 1 {
            xs.push(f64::from(*length).ln());
            ys.push(f64::from(*count).ln());
        }
    }

    let xs = Array1::from(xs);
    let ys = Array1::from(ys);

    let xs_mean = xs.mean().unwrap();
    let ys_mean = ys.mean().unwrap();

    let xs_centered = &xs - xs_mean;
    let ys_centered = &ys - ys_mean;

    let a = xs_centered.dot(&ys_centered) / xs_centered.dot(&xs_centered);
    let b = ys_mean - a * xs_mean;

    (a, b)
}

// use the parameters we got from the distribution to test if the distribution follow the power_law distribution 
fn test_power_law_fit(distribution: &HashMap<u32, u32>) -> bool {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for (length, count) in distribution {
        if *length > 1 {
            xs.push(f64::from(*length).ln());
            ys.push(f64::from(*count).ln());
        }
    }

    let (alpha, beta) = fit_power_law(distribution);
    let xmin = (1.0 / beta).exp();

    // Check if the slope is negative (alpha < 0) and the xmin is greater than 1
    alpha < 0.0 && xmin > 1.0
}


fn main() {
    // Read roadNet-CA.txt into a Graph
    let file = File::open("roadNet-CA.txt").expect("Unable to open the file");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read the line");
        let mut parts = line.split_whitespace();

        let u = parts.next().expect("Missing source vertex")
            .parse().expect("Invalid source vertex");
        let v = parts.next().expect("Missing destination vertex")
            .parse().expect("Invalid destination vertex");

        if parts.next().is_some() {
            panic!("Line contains more than two integers: {:?}", line);
        }

        graph.add_edge(u, v);
    }
    
    
    // Calculate shortest path lengths for a random vertex
    let random_vertex1 = 42;
    let random_vertex2 = 57;
    let distances = graph.bfs(random_vertex1);

    if let Some(distance1) = distances.get(&random_vertex1) {
        println!("Shortest Path Distance from vertex {} to random_vertex1: {}", random_vertex1, distance1);
    } else {
        println!("No shortest path distance found for random_vertex1: {}", random_vertex1);
    }

    if let Some(distance2) = distances.get(&random_vertex2) {
     println!("Shortest Path Distance from vertex {} to random_vertex2: {}", random_vertex2, distance2);
    } else {
     println!("No shortest path distance found for random_vertex2: {}", random_vertex2);
    }


    // Calculate the average shortest path length
    let mut sum: f64 = 0.0;
    let count = distances.len();
    for distance in distances.values() {
        sum += *distance as f64;
    }
    let average = sum / count as f64;
    println!("Average shortest path length: {}", average);

    // Analyze the distribution of shortest path lengths
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    for distance in distances.values() {
       *distribution.entry(*distance).or_insert(0) += 1;
    }
    println!("Shortest Path Length Distribution:");
    //println!("Distribution of shortest path lengths:");
    for (length, count) in &distribution {
        println!("Length {}: {}", length, count);
    }   

    // Plot the distribution of shortest path lengths
   plotter::plot_distribution(&distribution, "distribution.png").expect("Unable to plot distribution");


    // Test if the input distribution fits a power law
    let is_power_law_fit = test_power_law_fit(&distribution);

    // Print the result
    if is_power_law_fit {
        println!("The input distribution fits a power law.");
    } else {
        println!("The input distribution does not fit a power law.");
    }

    // Find highly connected vertices that serve as hubs for the shortest paths
    let hubs = find_hubs(&graph);
    println!("Highly connected vertices:");
    for (vertex, degree) in &hubs[0..10] {
    println!("Vertex {}: degree {}", vertex, degree);
    }


}
