// Here are  tests for the graph function and the fit_power_law function 

#[cfg(test)]
mod tests {
    use super::*;
    // Import the items from the parent module
    use super::*;
    use crate::Graph;
    use crate::fit_power_law;
    use std::collections::HashMap;

    #[test]
    //The test checks if the returned distances HashMap has the correct distance values for vertices 1, 2, 3, and 4. 
    fn test_graph() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let distances = graph.bfs(1);
        assert_eq!(distances.get(&1), Some(&0));
        assert_eq!(distances.get(&2), Some(&1));
        assert_eq!(distances.get(&3), Some(&2));
        assert_eq!(distances.get(&4), Some(&3));
    }

    #[test]
    // The test checks whether alpha is less than 0.0 and xmin is greater than 0.0. 
    // If both conditions are met, the test passes, indicating that the fit_power_law function correctly estimates the power-law parameters from the input distribution.
    fn test_fit_power_law() {
        let mut distribution: HashMap<u32, u32> = HashMap::new();
        distribution.insert(1, 100);
        distribution.insert(2, 50);
        distribution.insert(3, 25);
        distribution.insert(4, 12);
        distribution.insert(5, 6);

        let (alpha, xmin) = fit_power_law(&distribution);
        assert!(alpha < 0.0);
        assert!(xmin > 0.0);
    }
}