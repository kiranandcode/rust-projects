mod matrix;
use matrix::Matrix2D;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: usize,
    edges: Matrix2D,
    symmetric: bool,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph {}:\n", self.nodes);
        write!(f, "{}", self.edges)
    }
}



impl Graph {
    pub fn new(nodes: usize, symmetric: bool) -> Self {
        Graph {
            nodes,
            symmetric,
            edges: Matrix2D::new(nodes, nodes)
        }
    }

    pub fn set_edge(&mut self, from: usize, to: usize, weight: u32) {
        if from != to {
            self.edges[(from,to)] = weight;

            if self.symmetric {
                self.edges[(to,from)] = weight;
            }
        }
    }

    pub fn edge(&self, from: usize, to: usize) -> Option<u32> {
        if from == to {
            return None;
        }
        let result = self.edges[(from,to)];

        if result > 0 {
            Some(result)
        } else {
            None
        }
    }

    pub fn adjacent_nodes(&self, node: usize) -> Vec<usize> {
        let mut result = Vec::new();

        for other in 0..self.nodes {
            if self.edges[(node,other)] > 0 {
                result.push(other);
            }
        }
        return result;
    }
}


/// Naive shortest hamiltonian circuit calculator
fn find_shortest_hamiltonian_path(graph: &Graph, path: Vec<usize>, count: u32) -> Option<(u32, Vec<usize>)> {
    let mut nodes_to_try = Vec::new();

    // if we've formed a complete circuit, check if it works
    if path.len() == graph.nodes {
        if graph.edge(path[path.len()-1], path[0]).is_some() {
            return Some((count + graph.edge(path[path.len() -1], path[0]).unwrap(), path));
        } else {
            return None;
        }
    }

    let current_head = path[path.len() - 1];

    // find which nodes to try
    for i in 0..graph.nodes {
        if !path.contains(&i) && graph.edge(current_head, i).is_some() {
            nodes_to_try.push(i);
        }
    }

    let mut best_seen = None;
    // for each node, recurse, and try
    for node in nodes_to_try {
        let mut vector = path.clone();
        vector.push(node);
        let new_count = count + graph.edge(current_head, node).unwrap();

        match find_shortest_hamiltonian_path(graph, vector, new_count) {
            Some((cost, result_path)) => {
                match best_seen.take() {
                    Some((old_cost, old_result_path)) => {
                        if old_cost > cost {
                            best_seen = Some((cost, result_path));
                        } else {
                            best_seen = Some((old_cost, old_result_path));
                        }
                    },
                    None => {
                        best_seen = Some((cost, result_path));
                    }
                }
            },
            None => ()
        }
    }

    return best_seen;
}


fn main() {
    let mut graph = Graph::new(5,true);

    graph.set_edge(0, 1, 3);
    graph.set_edge(0, 2, 1);
    graph.set_edge(0, 3, 4);
    graph.set_edge(0, 4, 4);


    graph.set_edge(1, 2, 2);
    graph.set_edge(1, 3, 2);
    graph.set_edge(1, 4, 1);

    graph.set_edge(2, 3, 3);
    graph.set_edge(2, 4, 5);

    graph.set_edge(3, 4, 4);

    println!("{}", graph);
    let path = vec![0];

    println!("{:?}", find_shortest_hamiltonian_path(&graph, path, 0));
}
