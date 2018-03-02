pub mod ride;
use self::ride::Ride;
use std::vec::Vec;

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

}
