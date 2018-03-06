extern crate rand;
pub mod ride;
pub mod solution;
use self::ride::Ride;
use self::solution::Solution;
use std::vec::Vec;
use self::rand::Rng;
use super::matrix::Matrix;

#[derive(Debug)]
pub struct Problem {
    rows : i32,
    columns : i32,
    vehicles : i32,
    no_rides : i32,
    per_ride_bonus : i32,
    total_time : i32,
    rides : Vec<Ride>
}

#[derive(Debug)]
struct DFSResult {
    path: Vec<i32>
}

fn dfs(no_rides : i32, edge_matrix : &Matrix<i32>, rides : &Vec<Ride>, seen : &mut Vec<bool>) -> DFSResult {
        let null = Ride::new(0,0,0,0,0,-100,0);

        let mut parent: Vec<Option<i32>> = vec![None; no_rides as usize];
        let mut cost : Vec<Option<i32>>= vec![None; no_rides as usize];
        let mut added : Vec<bool>= vec![false; no_rides as usize];

        cost[0] = Some(0);
        let mut added_nodes = 0;
        let mut leaves = Vec::new();
        // while all nodes haven't been considered
        while(added_nodes != no_rides) {
            let mut max= None;
            let mut max_index = None;

            // find the maximum unadded node we can add
            for i in 0..no_rides {
                if !added[i as usize] && cost[i as usize].is_some() {
                    if max.is_none() || max.unwrap() < cost[i as usize] {
                            max = Some(cost[i as usize].clone()); 
                            max_index = Some(i);
                    }
                }
            }

            // if there is a node that we haven't added 
            match max{
                Some(maximum) => {
                    added[max_index.unwrap() as usize] = true;
                    added_nodes = added_nodes + 1;
                }
                None => { break; /* else break */ }
            }


            let max_index = max_index.unwrap();
            // now update all unseen costs
            // for each node
            let mut child_found = false;
            for i in 0..no_rides {
                // if we haven't added it
                if !added[i as usize] {
                    // find out the cost from the added node to it
                    let graph_value: i32;
                    unsafe {
                        graph_value = edge_matrix.get_unchecked(max_index as usize,i as usize).clone().into();
                    }
                    // if there was no edge to it before, but with this node there is
                    if cost[i as usize].is_none() && graph_value != 0 {
                        // update our records to list this node as being the accessible from the
                        // added node
                        parent[i as usize] = Some(max_index);
                        // the cost of this node should be edge weight + weights to get to
                        // max_index node
                        cost[i as usize] = Some(cost[max_index as usize].unwrap() + graph_value);
                        child_found = true;
                    }
                    
                    // else if we have seen the node, but the cost is now greater
                    else if !cost[i as usize].is_none() && cost[i as usize].unwrap() != 0 && graph_value + cost[max_index as usize].unwrap() > cost[i as usize].unwrap() {
                        // update the parent of this node to be the ones
                        cost[i as usize] = Some(graph_value + cost[max_index as usize].unwrap());
                        parent[i as usize] = Some(max_index);
                        child_found = true;
                    }
                }
            }

            if !child_found {
                leaves.push(max_index);
            }

        }

        // at this point djikstra has completed and we have a list of leaves
        // go through and find the leaves with the highest cost
       let mut highest = leaves[0]; 
       let mut highest_cost = cost[leaves[0] as usize];

       // find the highest_cost_leaf
       for i in 0..leaves.len() {
            let node = leaves[i];
            if let Some(score) = cost[node as usize] {
                if let Some(highest_score) = highest_cost {
                   if highest_score < score {
                        highest = node;
                        highest_cost= cost[node as usize].clone();
                   }
                } else {
                    highest = node;
                    highest_cost= cost[node as usize].clone();
                }
            }
       }

       // now just ascend from the leaf to the start
       let mut current_node = highest;
       let mut result = Vec::new();
       while current_node != 0 {
           result.push(current_node); 
           current_node = parent[current_node as usize].unwrap();
       }

       result.reverse();

       let result = DFSResult {
           path: result
        };

        println!("Result: {:?}", result);
        result
}

impl Problem {
    pub fn new(
        rows : i32,
        columns : i32,
        vehicles:  i32,
        no_rides: i32,
        per_ride_bonus :i32,
        total_time : i32,
        rides: Vec<Ride>) -> Problem {
        Problem {
            rows,
            columns,
            vehicles,
            no_rides,
            per_ride_bonus,
            total_time,
            rides
        }
    }

    pub fn solve(&self) -> Solution {
        let mut assignment : Vec<Vec<Ride>>= Vec::new();

        for i in 0..self.vehicles {
            assignment.push(Vec::new());
        }

        let mut rng = rand::thread_rng();
        let mut weights : Matrix<i32> =  Matrix::new((self.no_rides + 1) as usize, (self.no_rides + 1) as usize);
        for i in 0..self.no_rides {
            for j in 0..self.no_rides {
                if i != j {
                    if Ride::are_connected(&self.rides[i as usize], &self.rides[j as usize]) {
                        unsafe{
                            *weights.get_mut_unchecked((i+1) as usize,(j+1) as usize) = Ride::get_weight(&self.rides[i as usize], &self.rides[j as usize]);
                        }
                    }
                }
            }
        }

        let null = Ride::new(0,0,0,0,0,-100,0);

        for i in 0..self.no_rides {
            if Ride::are_connected(&null, &self.rides[i as usize]) {
                    unsafe{
                        *weights.get_mut_unchecked(0,(i+1) as usize) = Ride::get_weight(&null, &self.rides[i as usize]);
                    }
            }
        }


        // Do DFS method to find leaves
        let result = dfs(self.no_rides + 1, &weights, &self.rides);

        for i in 0..self.no_rides {
            let bucket = (rng.gen::<i32>() % self.vehicles).abs();
            assignment[bucket as usize].push(self.rides[i as usize].clone());

        }

        Solution::new(assignment)
    }

}
