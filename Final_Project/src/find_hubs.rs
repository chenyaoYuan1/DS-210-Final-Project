// Here is the function that eturns a vector of tuples representing the vertices and their corresponding degrees. 
use std::collections::{HashMap};
use crate::Graph;

pub fn find_hubs(graph: &Graph) -> Vec<(u32, usize)> {
    let mut degrees: HashMap<u32, usize> = HashMap::new();

    for (vertex, neighbors) in &graph.adjacency_list {
        degrees.insert(*vertex, neighbors.len());
    }

    let mut hubs: Vec<(u32, usize)> = degrees.into_iter().collect();
    hubs.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));
    hubs
}
