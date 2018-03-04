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
                if Ride::are_connected(&self.rides[i as usize], &self.rides[j as usize]) {
                    unsafe{
                        *weights.get_mut_unchecked((i+1) as usize,(j+1) as usize) = Ride::get_weight(&self.rides[i as usize], &self.rides[j as usize]);
                    }
                }
            }
        }

        let null = Ride::new(0,0,0,0,0,0,0);

        for i in 0..self.no_rides {
            if Ride::are_connected(&null, &self.rides[i as usize]) {
                    unsafe{
                        *weights.get_mut_unchecked(0,(i+1) as usize) = Ride::get_weight(&null, &self.rides[i as usize]);
                    }
            }
        }

        for i in 0..self.no_rides {
            let bucket = (rng.gen::<i32>() % self.vehicles).abs();
            assignment[bucket as usize].push(self.rides[i as usize].clone());

        }

        Solution::new(assignment)
    }

}
