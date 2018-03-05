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

        let mut nearest = vec![None; no_rides];
        let mut cost = vec![None; no_rides];
        let mut added = vec![false; no_rides];

        cost[start] = 0;
        let mut added_nodes = 0;
        while(added_nodes != self.nodes) {
            let mut max= None;
            let mut u = None;

            for i in 0..no_rides {
                if !added[i] && cost[i].is_present() {
                    if min.is_none() || max.unwrap() < cost[i] {
                            max = Some(cost[i].clone()); 
                            u = Some(i);
                    }
                }
            }

            let u = u.unwrap();
            match max{
                Some(maximum) => {
                    added[u] = true;
                    added_nodes = added_nodes + 1;
                }
                None => { break; }
            }

            for i in 0..no_rides {
                if !added[i] {
                    let graph_value: i32;
                    unsafe {
                        graph_value = self.graph.get_unchecked(u,i).clone().into();
                    }
                    if cost[i].is_none() && graph_value != 0 {
                        nearest[i] = Some(u);
                        cost[i] = cost[u] + graph_value;
                    } else if !cost[i].is_none() && cost[i].unwrap() != 0 && graph_value + cost[u].unwrap() < cost[i].unwrap() {
                        cost[i] = graph_value + cost[u];
                        nearest[i] = Some(u);
                    }
                }
            }

        }
       let result = DFSResult {
            children,
            parents
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
