
#[derive(Debug,Clone)]
pub struct Ride {
    pub start_X : i32,
    pub start_Y : i32,
    pub end_X : i32,
    pub end_Y : i32,
    pub earliest_start : i32,
    pub latest_end : i32,
    pub duration : i32,
    
    pub latest_start : i32,
    pub earliest_end : i32, i : i32
}

fn distance(ride_a : &Ride, ride_b : &Ride) -> i32 {
    return (ride_a.end_X - ride_b.start_X).abs() + (ride_a.end_Y - ride_b.start_Y).abs();
}

fn time_difference(ride_a : &Ride, ride_b : &Ride) -> i32 {
    if (ride_b.earliest_end - ride_a.earliest_start).abs() < (ride_b.latest_end - ride_a.earliest_start).abs() {
        ride_b.earliest_end - ride_a.earliest_start
    } else {
        ride_b.latest_end - ride_b.earliest_end + (ride_b.latest_end - ride_a.earliest_start)
    }
}

fn time_difference_given_time(current_time : i32, ride_a : &Ride, ride_b : &Ride) -> i32 {
        if current_time < ride_a.earliest_start {
            ride_b.earliest_start - (ride_a.earliest_start + ride_a.duration)
        } else {
            ride_b.earliest_start - (current_time + ride_a.duration)
        }
}

fn invert(inp : i32) -> i32 {

    let result = if inp ==  0 {
        1000001
    } else {
        ((1 as f64 /inp as f64) * 100000 as f64) as i32
    };

    result
}

impl Ride {
    pub fn new(
        i:i32,
        start_X : i32,
        start_Y : i32,
        end_X : i32,
        end_Y : i32,
        earliest_start : i32,
        latest_end : i32) -> Ride {
        let duration = (start_X - end_X).abs() +  (start_Y - end_Y).abs();
             
        Ride {
            start_X,
            start_Y,
            end_X,
            end_Y,
            earliest_start,
            latest_end,
            duration,
            latest_start: latest_end - duration,
            earliest_end: earliest_start + duration,
            i : i
        }
    }

    pub fn get_duration(&self) -> i32 {
        self.duration
    }

    pub fn getId(&self) -> i32 {
        self.i
    }

    pub fn distance_between(ride_a : &Ride, ride_b : &Ride) -> i32 {
        return distance(ride_a, ride_b);
    }
    pub fn are_connected(ride_a : &Ride, ride_b : &Ride) -> bool {
        return ride_a.earliest_end + distance(ride_a, ride_b) < ride_b.latest_start; 
    }

    pub fn are_connected_given_time(current_time : i32, ride_a : &Ride, ride_b : &Ride) -> bool {
        if current_time < ride_a.earliest_start {
            ride_a.earliest_start + ride_a.duration + distance(ride_a, ride_b) < ride_b.latest_start
        } else {
            current_time + ride_a.duration + distance(ride_a, ride_b) < ride_b.latest_start
        }
    }

    pub fn get_time_after_completion(&self, current_time : i32, start_ride : &Ride) -> i32 {
        if current_time > self.latest_start {
            panic!("Current time exceeds latest start time - means bad calculation");
        }
        let result_time = current_time + distance(start_ride, self) + self.duration;
        if result_time > self.latest_end {
            panic!("Finish time exceeds latest end time - means bad calculation");
        }

        result_time
    }

    pub fn get_weight(ride_a : &Ride, ride_b : &Ride) -> i32 {
        return invert(distance(ride_a, ride_b)) +  invert(time_difference(ride_a, ride_b)) + ride_b.duration;
    }

    pub fn get_weight_given_time(current_time : i32, ride_a : &Ride, ride_b : &Ride) -> i32 {
        return invert(distance(ride_a, ride_b)) + invert(time_difference_given_time(current_time, ride_a, ride_b)) + ride_b.duration;
    }
}
