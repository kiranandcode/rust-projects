extern crate rand;
pub mod ride;
pub mod solution;
use self::ride::Ride;
use self::solution::Solution;
use std::vec::Vec;
use self::rand::Rng;

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

        for i in 0..self.no_rides {
            let bucket = rng.gen::<i32>() %self.vehicles;
            assignment[bucket as usize].push(self.rides[i as usize].clone());

        }

        Solution::new(assignment)
    }

}
