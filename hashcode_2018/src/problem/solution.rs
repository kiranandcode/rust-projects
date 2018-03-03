use super::ride::Ride;
use super::Problem;
use std::fmt::{Debug, Display, Formatter, Write, self};
pub struct Solution{
    assignment : Vec<Vec<Ride>>
}


impl Solution {
    pub fn new(assignment : Vec<Vec<Ride>>) -> Solution {
        Solution {
            assignment
        }
    }

}

impl  Display for Solution {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
       let mut buffer = String::new(); 


       for (i, rides) in self.assignment.iter().enumerate() {
            write!(buffer, "{}", i);

           for ride in rides.iter() {
                write!(buffer, " {}", ride.getId());    
           }
           write!(buffer, "\n");
       }

       write!(f, "{}", buffer)
    }
}
