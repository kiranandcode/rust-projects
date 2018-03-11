extern crate rand;
use self::rand::Rng;

extern crate chan_signal;
pub mod ride;
pub mod solution;
use self::ride::Ride;
use self::solution::Solution;
use std::vec::Vec;
use super::matrix::Matrix;
use self::chan_signal::Signal;
use std::sync::{Mutex,Arc};
use std::thread;

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

fn dfs(no_rides : i32, edge_matrix : &Matrix<i32>, rides : &Vec<Ride>, seen : &mut Vec<bool>, seen_count : &mut i32, total_time : i32, bonus : i32) -> Option<DFSResult> {
        let null = Ride::new(0,0,0,0,0,-100,0);

        // initialize temporary variables
            // parent : mapping of nodes -> parents
        let mut parent: Vec<Option<i32>> = vec![None; no_rides as usize];
            // cost : mapping of node -> cost to get to node
        let mut cost : Vec<Option<i32>>= vec![None; no_rides as usize];
        let mut current_time : Vec<i32>= vec![0; no_rides as usize];
            // added : whether a node has been added to the dfs tree (seen nodes are already added)
        let mut added : Vec<bool>= seen.clone();
        added[0] = false;

        cost[0] = Some(0);
        let mut added_nodes = 0 + *seen_count;
        let mut leaves = Vec::new();
        // while all nodes haven't been considered (there is early termination built in)
        //while(added_nodes != no_rides) {
        loop {
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
            let max_index_time = current_time[max_index as usize];
                for i in 0..no_rides {

                    // if we haven't added it
                    if !added[i as usize] {
                        let mut completed_time =  max_index_time;
                        // find out the cost from the added node to it
                        let mut graph_value: i32 = 0;
                       let mut is_connected; 
                       let start_ride = if max_index > 0 {
                            &rides[(max_index - 1) as usize]
                       } else {
                            &null
                       };
                       let end_ride = if i > 0 {
                            &rides[(i-1) as usize]
                       } else {
                            &null
                       };

//                        if i > 0  && max_index > 0 {
                            is_connected = Ride::are_connected_given_time(max_index_time, start_ride, end_ride);
                            if is_connected {
                                graph_value = Ride::get_weight_given_time(max_index_time, start_ride, end_ride, bonus);
                                completed_time = end_ride.get_time_after_completion(current_time[max_index as usize], start_ride);
                                if completed_time + end_ride.duration  >= total_time {
                                    is_connected = false;
                                }
                            }
                       /* } else {
                            is_connected = true;
                            unsafe {
                                graph_value = edge_matrix.get_unchecked(max_index as usize,i as usize).clone().into();
                            }
                        }*/


                        // if there was no edge to it before, but with this node there is
                        if cost[i as usize].is_none() && graph_value != 0 && is_connected {
                            // update our records to list this node as being the accessible from the
                            // added node
                            parent[i as usize] = Some(max_index);
                            // the cost of this node should be edge weight + weights to get to
                            // max_index node
                            cost[i as usize] = Some(cost[max_index as usize].unwrap() + graph_value);
                            //if i > 0 && max_index > 0 {
                                current_time[i as usize] = completed_time;
                            //}
                            child_found = true;
                        }
                        
                        // else if we have seen the node, but the cost is now greater
                        else if !cost[i as usize].is_none() && cost[i as usize].unwrap() != 0 && graph_value + cost[max_index as usize].unwrap() > cost[i as usize].unwrap() && is_connected {
                            // update the parent of this node to be the ones
                            //
                            cost[i as usize] = Some(graph_value + cost[max_index as usize].unwrap());
                            if i > 0 && max_index > 0 {
                                current_time[i as usize] = rides[(i - 1) as usize].get_time_after_completion(current_time[max_index as usize], &rides[(max_index -1) as usize]);
                            }
     
                            parent[i as usize] = Some(max_index);
                            child_found = true;

                        }
                    }
                }

                // if the node had no children push it
                if !child_found {
                    leaves.push(max_index);
                }
        }

        // at this point djikstra has completed and we have a list of leaves
        // go through and find the leaves with the highest cost
        //
        if leaves.len() == 0 {
            return None;
        }


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
           seen[current_node as usize] = true;
           *seen_count = *seen_count + 1;
           current_node = parent[current_node as usize].unwrap();
       }

       result.reverse();

       let result = DFSResult {
           path: result
        };

        Some(result)
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

    pub fn solve(&self, low_expt_max : i32, high_expt_max : i32) -> Solution {
        let mut assignment : Vec<Vec<Ride>>= Vec::new();

        // set  up the weights matrix for later in the problem
        let mut rng = rand::thread_rng();
        let mut weights : Matrix<i32> =  Matrix::new((self.no_rides + 1) as usize, (self.no_rides + 1) as usize);
        for i in 0..self.no_rides {
            for j in 0..self.no_rides {
                if i != j {
                    if Ride::are_connected(&self.rides[i as usize], &self.rides[j as usize]) {
                        unsafe{
                            *weights.get_mut_unchecked((i+1) as usize,(j+1) as usize) = Ride::get_weight(&self.rides[i as usize], &self.rides[j as usize], self.per_ride_bonus);
                        }
                    }
                }
            }
        }

        // also add weights from special null node at (0,0) to all connected nodes
        let null = Ride::new(0,0,0,0,0,-100,0);
        for i in 0..self.no_rides {
            if Ride::are_connected(&null, &self.rides[i as usize]) {
                    unsafe{
                        *weights.get_mut_unchecked(0,(i+1) as usize) = Ride::get_weight(&null, &self.rides[i as usize], self.per_ride_bonus);
                    }
            }
        }


        // Do DFS method to find leaves
        let mut seen = vec![false; ((self.no_rides + 1) as usize)];
        let mut seen_count = 0;

        // shared flag to track user requested early termination
        let should_break_early = Arc::new(Mutex::new(false));
        // set up channel to listen for sigint signals from terminal
        let signal = chan_signal::notify(&[Signal::INT, Signal::TERM, Signal::IO]);

        // create new binding (so that move will move it) for the thread to use to access the
        // shared variable
        let thread_should_break_early = should_break_early.clone();
        // use thread to handle sigint calls
        thread::spawn(move || {
            // loop forever
            loop {
                // retrieve inputs from signal listener
                let result = signal.recv();
                if let Some(signal) = result {
                    println!("Early Termination Requested. Will terminate as soon as current DFS loop ends");
                    // if input recieved, update internal variable
                    let mut data = thread_should_break_early.lock().unwrap();
                    *data = true;
                    break;
                }
            }
        });

        let mut last_seen_count = 0;
            for i in 0..self.no_rides {
                // at the beginning, check value of shared variable - if set break
                {
                    let mut data = should_break_early.lock().unwrap();
                    if *data {
                        break;
                    }
                }
                
                println!("Iteration[{}]: allocated {} (+{})", i, seen_count, seen_count-last_seen_count);
                last_seen_count = seen_count;
                let path = match dfs((self.no_rides + 1), &weights, &self.rides, &mut seen, &mut seen_count, self.total_time, self.per_ride_bonus) {
                    Some(result) => {
                        result.path.into_iter().map(|x| self.rides[(x - 1) as usize].clone()).collect()
                    }
                    None => {
                        Vec::new()
                    }
                };
                assignment.push(path);
                // let bucket = (rng.gen::<i32>() % self.vehicles).abs();
                // assignment[bucket as usize].push(self.rides[i as usize].clone());

            }

        println!("Main Loop Terminated, Writing solution to file");


        Solution::new(assignment)
    }

}
