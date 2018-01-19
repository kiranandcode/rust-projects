use matrix::Matrix;

use regex::Regex;

use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write, self};

#[derive(Debug)]
pub struct Graph<T> {
    graph: Matrix<T>,
    nodes: usize
}

#[derive(Debug)]
pub struct PathMatrix {
    root: usize,
    nodes: usize,
    nearest: Vec<Option<usize>>,
    costs: Vec<i32>
}

impl<T> Graph<T>
    where T : Default + FromStr + PartialEq {

        pub fn new(n: usize) -> Self {
            Graph {
                graph: Matrix::new(n,n),
                nodes: n
            }
        }

        pub fn add_edge(&mut self, from_edge : usize, to_edge: usize, weight: T) {
            let edge = self.graph.get_mut(from_edge, to_edge).unwrap();
            *edge = weight;
        }

        pub fn from_file<P>(filename: P) -> Result<Self,String> 
            where P : AsRef<Path>{
                    let file = try!(File::open(filename).map_err(|_e| "could not find file".to_owned()));
                    let buf = BufReader::new(file);
                    let mut lines = buf.lines();

                    let specification_regex = Regex::new(r"^\s*N:\s+(\d+)\s*E:\s+(\d+)\s*$").unwrap();
                    let edge_regex = Regex::new(r"^\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*$").unwrap();

                    let get_spec_result : Result<String, _> = try!(lines.next().ok_or("invalid graph file format - expecting specification on line 1".to_owned()));
                    let spec_str : String = try!(get_spec_result.map_err(|_e| { "invalid graph file format - expecting specification on line 1".to_owned() }));
                    let specification_captures = try!(specification_regex.captures(&spec_str).ok_or(format!("line [0]: invalid graph file format - expecting specification on line 1")));

                    
                    let nodes_str = &specification_captures[1];
                    let edges_str = &specification_captures[2];

                    let nodes : usize = try!(nodes_str.parse().map_err(|_e| "invalid graph file format - number of nodes is not an integer".to_owned()));
                    let edges : usize = try!(edges_str.parse().map_err(|_e| "invalid graph file format - number of edges is not an integer".to_owned()));

                    let mut graph = Graph::new(nodes);

                    for (i, line) in lines.take(edges).enumerate() {
                        let i = i + 1;
                        let line = try!(line.map_err(|_e| format!("line [{}]: invalid graph file format - not all edges present", i)));
                        // each line should be <(> <from> <,> <to> <)>
                        let edge_captures = try!(edge_regex.captures(&line).ok_or(format!("line [{}]: invalid graph file format - edge definition is malformed", i)));

                        let from_node_str = &edge_captures[1];
                        let to_node_str = &edge_captures[2];
                        let weight_str = &edge_captures[3];


                        let from_node : usize = try!(from_node_str.parse().map_err(|_e| "invalid graph file format - one `from-node` value is not an integer".to_owned()));
                        let to_node : usize = try!(to_node_str.parse().map_err(|_e| "invalid graph file format - one `to-node` value is not an integer".to_owned()));
                        let weight_node : T = try!(weight_str.parse().map_err(|_e| "invalid graph file format - one weight value is not of the right type".to_owned()));

                        graph.add_edge(from_node, to_node, weight_node);

                    }


                    Ok(graph)
        }

        pub fn dfs(&self, start : usize) -> PathMatrix {
            if(start > self.nodes) {
                panic!("graph error - start node {} is greater than no of nodes {}", start, self.nodes);
            }

            let mut parent = vec![None; self.nodes];
            let mut cost = vec![0; self.nodes];
            let mut onstack = vec![false; self.nodes];

            let mut stack = Vec::new();

            stack.push(start);
            onstack[start] = true;

            while let Some(mut node) = stack.pop() {
                onstack[node] = true;

                for i in 0..self.nodes {
                    let connectedNode;
                    unsafe {
                       connectedNode = *self.graph.get_unchecked(node, i) != T::default() && !onstack[i];
                    }
                        if connectedNode {
                            parent[i] = Some(node);
                            cost[i] = 1 + cost[node];
                            stack.push(i);
                            onstack[i] = true;
                        }
                }
            }

            PathMatrix {
                root: start,
                nodes: self.nodes,
                nearest: parent,
                costs: cost
            }
        }
} 

impl<T : Display + Debug> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer =  format!("Graph({}): ", self.nodes);
        write!(buffer, "{}", self.graph);
        write!(f, "{}", buffer)
    }
}