use matrix::Matrix;
use graphviz::GraphVizDiGraph;

use regex::Regex;
use std::cmp;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write, self};
use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Sub};


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

pub struct GraphPath<'a, 'b, T : 'a>(&'a Graph<T>, &'b PathMatrix);

impl<T> Graph<T>
    where T : Default {
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


}


impl<T> Graph<T>
    where T : Default + FromStr + PartialEq + PartialOrd + Clone {

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


}

impl<T> Graph<T>
    where T : Default +  PartialEq + PartialOrd + Clone {

       
        pub fn overlay_path<'a,'b>(&'a self, path : &'b PathMatrix) -> GraphPath<'a, 'b, T> {
            GraphPath(&self, path)
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

        pub fn bfs(&self, start: usize) -> PathMatrix {
            if(start > self.nodes) {
                panic!("graph error - start node {} is greater than no of nodes {}", start, self.nodes);
            }

            let mut parent = vec![None; self.nodes];
            let mut cost   = vec![0   ; self.nodes];
            let mut onstack = vec![false; self.nodes];
            let mut visited = vec![false; self.nodes];
            let mut queue = VecDeque::new();
            queue.push_back(start);

            while let Some(node) = queue.pop_front() {
                visited[node] = true;
                for i in 0..self.nodes {
                    let mut shouldAdd;
                    unsafe {
                        shouldAdd = *self.graph.get_unchecked(node, i) != T::default() && !visited[i] && !onstack[i];
                    }
                    if shouldAdd {
                        parent[i] = Some(node);
                        cost[i] = 1 + cost[i];
                        onstack[i] = true;
                        queue.push_back(i);
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

        pub fn create_graph_with_path(&self, matrix : &PathMatrix) -> Graph<T> {
            let mut graph = Graph::new(self.nodes);
            for i in 0..self.nodes {
                if matrix.nearest[i].is_some() {
                    unsafe {
                        *graph.graph.get_mut_unchecked(i,i) =  self.graph.get_unchecked(i,i).clone();
                    }
                }
            }

            graph
        }

        pub fn prenum_ordering(&self, root : usize) -> PathMatrix {
            if(root > self.nodes) {
                panic!("graph error - root node {} is greater than no of nodes {}", root, self.nodes);
            }
            
            let mut prenum = vec![0; self.nodes];
            let mut nearest = vec![None; self.nodes];
            let mut visited = vec![false; self.nodes];
            let mut stack = Vec::new();
            stack.push(root);
            let mut visit_count = 0;
            let mut num = 0;

            while visit_count != self.nodes {
               while let Some(node) = stack.pop() {
                   if !visited[node] {
                       visited[node] = true;

                       visit_count += 1;
                       num += 1;
                       prenum[node] = num;

                       for i in 0..self.nodes {
                           unsafe {
                           if *self.graph.get_unchecked(node, i) != T::default() && !visited[i] {
                                stack.push(i);
                                nearest[i] = Some(node); 
                           }
                        }
                       }
                   }
               }


               let mut notvisit = None;

               for i in 0..self.nodes {
                   if !visited[i] {
                       notvisit = Some(i);
                       break;
                   }
               }

               match(notvisit) {
                   Some(value) => {
                       stack.push(value);
                   }
                   None => {
                       break;
                   }
               }
            }

            PathMatrix {
                root: root,
                costs: prenum,
                nearest: nearest,
                nodes: self.nodes,
            }
        }


        pub fn postnum_ordering(&self, root : usize) -> PathMatrix {
            if(root > self.nodes) {
                panic!("graph error - root node {} is greater than no of nodes {}", root, self.nodes);
            }
            
            let mut postnum = vec![0; self.nodes];
            let mut nearest = vec![None; self.nodes];

            let mut visited = vec![false; self.nodes];
            let mut stack = Vec::new();

            stack.push(root);

            let mut visit_count = 0;
            let mut num = 0;

            while visit_count != self.nodes {
               while let Some(&node) = stack.last() {
                   if !visited[node] {
                       visit_count += 1;
                       let mut added = 0;

                       for i in 0..self.nodes {
                           unsafe {
                           if *self.graph.get_unchecked(node, i) != T::default() && !visited[i] {
                               added += 1;
                                stack.push(i);
                                if nearest[i].is_none() {
                                    nearest[i] = Some(node); 
                                }
                           }
                        }
                       }

                       if added == 0 {
                            visited[node] = true;
                           num += 1;
                           postnum[node] = num;
                       }
                   } else {
                       stack.pop();
                   }
               }


               let mut notvisit = None;

               for i in 0..self.nodes {
                   if !visited[i] {
                       notvisit = Some(i);
                       break;
                   }
               }

               match(notvisit) {
                   Some(value) => {
                       stack.push(value);
                   }
                   None => {
                       break;
                   }
               }
            }

            PathMatrix {
                root: root,
                costs: postnum,
                nearest: nearest,
                nodes: self.nodes,
            }
        }



        pub fn generate_flow_to(&self, matrix : &PathMatrix, end : usize) -> Option<Graph<T>> {
            if(matrix.nearest[end] == None)  {
                return None; 
            }

            let mut end  = end;
            let mut graph : Graph<T> = Graph::new(self.nodes);
            let mut min : Option<&T> = None;
            let mut current = end;

            // working from the end of the graph, work backwards finding the minimum cost end on the path
            while(end != matrix.root) {
               unsafe {
                    if(min == None || min.unwrap() > self.graph.get_unchecked(matrix.nearest[end].unwrap(), end)) {
                        min = Some(self.graph.get_unchecked(matrix.nearest[end].unwrap(), end));
                    }
                }
                end = matrix.nearest[end].unwrap();
                if(matrix.nearest[end].is_none()) {
                    return None;
                }
            }

            if(min.is_none()) {return None;}

            let min = min.unwrap();

            end = current;

            while(end != matrix.root) {
                unsafe {
                    *(graph.graph.get_mut_unchecked(matrix.nearest[end].unwrap(), end)) = min.clone();
                    end = matrix.nearest[end].unwrap();
                }
            }

            return Some(graph);
        }

} 

impl<T> Graph<T>
    where T : Default + PartialEq + PartialOrd + Add<Output = T> + Sub<Output = T> + Clone,
    {
        pub fn augment_graph(capacity_graph : &Graph<T>, f: &Graph<T>, fstar: &Graph<T>) -> Graph<T> {
            let mut augmented = Graph::new(f.nodes);
            
            for i in 0..f.nodes {
                for j in 0..f.nodes {
                    let mut val = T::default();
                    unsafe {
                        if *capacity_graph.graph.get_unchecked(i,j) != T::default() {
                            val = f.graph.get_unchecked(i,j).clone() + fstar.graph.get_unchecked(i,j).clone()  - fstar.graph.get_unchecked(j,i).clone();
                        }

                        *augmented.graph.get_mut_unchecked(i,j) = val;
                    }
                }
            }

            augmented
        }

        pub fn generate_residual_flow_graph(capacity_graph : &Graph<T>, flow_graph: &Graph<T>) -> Graph<T> {
            let mut residual = Graph::new(capacity_graph.nodes);
            for i in 0..capacity_graph.nodes {
                for j in 0..capacity_graph.nodes {
                    let val;
                    unsafe {

                        if *capacity_graph.graph.get_unchecked(i,j) != T::default() {
                            val = capacity_graph.graph.get_unchecked(i,j).clone() - flow_graph.graph.get_unchecked(i,j).clone() ;
                        } else if *capacity_graph.graph.get_unchecked(j,i) != T::default() {
                            val = flow_graph.graph.get_unchecked(j,i).clone() ;
                        } else {
                            val = T::default();
                        }
                        *residual.graph.get_mut_unchecked(i,j) = val;
                    }
                }
            }

            residual
        }


        pub fn ford_fulkerson(capacity_graph : &Graph<T>, source : usize, sink : usize) -> Graph<T>  {
                let mut dfs_path = capacity_graph.dfs(source);
                let mut best_flow : Graph<T> = capacity_graph.generate_flow_to(&dfs_path, sink).expect("Could not form path from source to sink");
                let mut residual = Graph::generate_residual_flow_graph(&capacity_graph, &best_flow);

                dfs_path = residual.dfs(source);
                let mut new_flow = capacity_graph.generate_flow_to(&dfs_path, sink);

                while let Some(new_flow_graph) = new_flow {
                    let better_flow = Graph::augment_graph(&capacity_graph, &best_flow, &new_flow_graph);
                    residual = Graph::generate_residual_flow_graph(&capacity_graph, &better_flow);
                    best_flow = better_flow;
                    dfs_path = residual.dfs(source);
                    new_flow = capacity_graph.generate_flow_to(&dfs_path, sink);
                }

                best_flow
        }

}

impl<T> Graph<T> 
    where T : Default + FromStr + PartialEq + PartialOrd + Clone + Into<i32> {
        pub fn djikstra(&self, start : usize) -> PathMatrix {
            if(start > self.nodes) {
                panic!("graph error - start node {} is greater than no of nodes {}", start, self.nodes);
            }


            let mut nearest = vec![Some(start); self.nodes];
            let mut cost = vec![-1; self.nodes];
            let mut added = vec![false; self.nodes];

            cost[start] = 0;
            let mut added_nodes = 0;

            while(added_nodes != self.nodes) {

                let mut min = None;
                let mut u = None;

                // find the lowest cost unseen node
                for i in 0..self.nodes {
                    if !added[i] && cost[i] != -1 {
                        if min.is_none() || min.unwrap() > cost[i] {
                            min = Some(cost[i].clone());
                            u = Some(i);
                        }
                    }
                }

                let u = u.unwrap();
                match(min) {
                    Some(minimum) => {
                        added[u] = true;
                        added_nodes = added_nodes + 1;
                    }
                    None => {break;}
                }


                // given the new node, update all other nodes
                for i in 0..self.nodes {
                    if !added[i] {
                        let graph_value : i32;
                        unsafe {
                            graph_value = self.graph.get_unchecked(u, i).clone().into();
                        }

                        if cost[i] == -1 && graph_value != 0 {
                            nearest[i] = Some(u);
                            cost[i] = cost[u] + graph_value;
                        } else if cost[i] != 0 && graph_value != 0 && graph_value + cost[u] < cost[i]  {
                            cost[i] = graph_value + cost[u];
                            nearest[i] = Some(u);
                        }
                    }
                }

            }
            


            PathMatrix {
                root: start,
                nodes: self.nodes,
                nearest: nearest,
                costs: cost
            }
        }


        pub fn prims(&self, start : usize) -> PathMatrix {
            if(start > self.nodes) {
                panic!("graph error - start node {} is greater than no of nodes {}", start, self.nodes);
            }


            let mut nearest = vec![Some(start); self.nodes];
            let mut cost = vec![-1; self.nodes];
            let mut added = vec![false; self.nodes];

            cost[start] = 0;
            let mut added_nodes = 0;

            while(added_nodes != self.nodes) {

                let mut min = None;
                let mut u = None;

                // find the lowest cost unseen node
                for i in 0..self.nodes {
                    if !added[i] && cost[i] != -1 {
                        if min.is_none() || min.unwrap() > cost[i] {
                            min = Some(cost[i].clone());
                            u = Some(i);
                        }
                    }
                }

                let u = u.unwrap();
                match(min) {
                    Some(minimum) => {
                        added[u] = true;
                        added_nodes = added_nodes + 1;
                    }
                    None => {break;}
                }


                // given the new node, update all other nodes
                for i in 0..self.nodes {
                    if !added[i] {
                        let graph_value : i32;
                        unsafe {
                            graph_value = self.graph.get_unchecked(u, i).clone().into();
                        }

                        if cost[i] == -1 && graph_value != 0 {
                            nearest[i] = Some(u);
                            cost[i] = graph_value;
                        } else if cost[i] != 0 && graph_value != 0 && graph_value < cost[i]  {
                            cost[i] = graph_value;
                            nearest[i] = Some(u);
                        }
                    }
                }

            }
            


            PathMatrix {
                root: start,
                nodes: self.nodes,
                nearest: nearest,
                costs: cost
            }
        }



}

impl<'a, 'b, T: Display + Debug + PartialOrd + Default> Display for GraphPath<'a, 'b, T> {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut renderer = GraphVizDiGraph::new("rendered_graph".to_owned());
            for i in 0..self.0.nodes {
                let node = renderer.with_node(format!("{}", i));
                if self.1.root == i {
                    node.with_attribute("peripheries".to_owned(), "2".to_owned());
                }
            }

            let mut hashSet : HashSet<(usize, usize)> = HashSet::new();
           for node in 0..self.0.nodes {
                if node != self.1.root && self.1.nearest[node].is_some() {
                    let mut node = node;
                    while node != self.1.root {
                        let edge_being_considered = (self.1.nearest[node].unwrap(), node);
                        if !hashSet.contains(&edge_being_considered) {
                            let mut edge = renderer.add_labelled_edge(format!("{}", self.1.nearest[node].unwrap()), format!("{}", node), format!("{}", self.1.costs[node]));
                            edge.with_attribute("color".to_owned(), "red".to_owned());
                            hashSet.insert(edge_being_considered);
                        }
                        match self.1.nearest[node] {
                            Some(value) => node = value,
                            None        => {
                                break;
                            }
                        }
                    }
                }
            }

            for node in 0..self.0.nodes {
                for other in 0..self.0.nodes {
                    let edge_being_considered = (node, other);
                    if !hashSet.contains(&edge_being_considered) {
                        unsafe {
                            let value = self.0.graph.get_unchecked(node, other);
                            if *value > T::default() {
                                renderer.add_labelled_edge(format!("{}", node), format!("{}", other), format!("{}", *value));
                            }
                        }
                            hashSet.insert(edge_being_considered);
                    }
                }
            }
 

            write!(f, "{}", renderer)
 
    }
}

impl<T : Display + Debug + PartialOrd + Default> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut renderer = GraphVizDiGraph::new("rendered_graph".to_owned());
        for i in 0..self.nodes {
            renderer.with_node(format!("{}", i));
        }

        for node in 0..self.nodes {
            for other in 0..self.nodes {
                unsafe {
                    let value = self.graph.get_unchecked(node, other);
                    if *value > T::default() {
                        renderer.add_labelled_edge(format!("{}", node), format!("{}", other), format!("{}", *value));
                    }
                }
            }
        }
        write!(f, "{}", renderer)
    }
}
