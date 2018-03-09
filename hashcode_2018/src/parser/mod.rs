use ::std::vec::Vec;
use ::std::fs::File;
use ::std::io::{Write, Read, Error, ErrorKind, BufReader, BufRead};

pub use hashcode_2018::problem::ride::Ride;
pub use hashcode_2018::problem::Problem;

pub fn read_input(name : &str) -> Problem {
    let mut input = File::open(format!("problems/{}.in", name)).expect("Could not open input file");
    let mut reader = BufReader::new(input);

    
        
     let rows : i32;
     let columns : i32;
     let vehicles : i32;
     let ride_count : i32;
     let per_ride_bonus : i32;
     let total_time : i32;




     let mut current_line : String = String::new();
     reader.read_line(&mut current_line);
     {
         let i_rows;
         let i_columns;
         let i_vehicles;
         let i_ride_count;
         let i_per_ride_bonus;
         let i_total_time;

         let mut iter  = current_line.split(char::is_whitespace);
         i_rows = iter.next().expect("No rows provided");
         i_columns = iter.next().expect("No columns provided");
         i_vehicles = iter.next().expect("No vehicles provided");
         i_ride_count = iter.next().expect("No number of rides provided");
         i_per_ride_bonus = iter.next().expect("No per ride bonus provided");
         i_total_time = iter.next().expect("No overall steps in simulation provided");

         rows = i_rows.parse().expect("rows was not a number");
         columns = i_columns.parse().expect("No columns provided");
         vehicles = i_vehicles.parse().expect("No vehicles provided");
         ride_count = i_ride_count.parse().expect("No ride_count of rides provided");
         per_ride_bonus = i_per_ride_bonus.parse().expect("No per_ride_bonus ride bonus provided");
         total_time = i_total_time.parse().expect("No total_time steps in simulation provided");
     }


     let mut rides = Vec::new();

     for i in 0..ride_count {
        current_line = String::new();
        reader.read_line(&mut current_line);
        let mut iter = current_line.split(char::is_whitespace);
        let start_X = iter.next().expect("No start_X provided");
        let start_Y = iter.next().expect("No start_Y provided");
        let end_X = iter.next().expect("No end_X provided");
        let end_Y = iter.next().expect("No end_Y provided");
        let earliest_start = iter.next().expect("No earliest_start provided");
        let latest_end = iter.next().expect("No latest_end provided");

        let start_X : i32 = start_X.parse().expect("No start_X provided");
        let start_Y : i32 = start_Y.parse().expect("No start_Y provided");
        let end_X : i32 = end_X.parse().expect("No end_X provided");
        let end_Y : i32 = end_Y.parse().expect("No end_Y provided");
        let earliest_start : i32 = earliest_start.parse().expect("No earliest_start provided");
        let latest_end : i32 = latest_end.parse().expect("No latest_end provided");

        let ride = Ride::new(i, start_X, start_Y, end_X, end_Y, earliest_start, latest_end);
        rides.push(ride); 
     }

    Problem::new(rows, columns, vehicles, ride_count, per_ride_bonus, total_time, rides)
}
