use std::collections::{HashMap, VecDeque};

// the Graph struct represents a graph data structure and provides methods to create a new graph, add edges between vertices. 
pub struct Graph {
    pub adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: u32, dest: u32) {
        self.adjacency_list
            .entry(src)
            .or_insert_with(Vec::new)
            .push(dest);
        self.adjacency_list
            .entry(dest)
            .or_insert_with(Vec::new)
            .push(src);
    }

// Perform a breadth-first search to calculate the shortest path distances from a given start vertex to all other vertices.
    pub fn bfs(&self, start_vertex: u32) -> HashMap<u32, u32> {
        let mut distances: HashMap<u32, u32> = HashMap::new();
        let mut queue: VecDeque<u32> = VecDeque::new();
        distances.insert(start_vertex, 0);
        queue.push_back(start_vertex);

        while !queue.is_empty() {
            let current_vertex = queue.pop_front().unwrap();
            let current_distance = *distances.get(&current_vertex).unwrap();

            if let Some(neighbors) = self.adjacency_list.get(&current_vertex) {
                for &neighbor in neighbors {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, current_distance + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        distances
    }
}
