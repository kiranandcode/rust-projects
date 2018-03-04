
#[derive(Debug,Clone)]
pub struct Ride {
    start_X : i32,
    start_Y : i32,
    end_X : i32,
    end_Y : i32,
    earliest_start : i32,
    latest_end : i32,
    duration : i32,
    
    latest_start : i32,
    earliest_end : i32, i : i32
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
    if inp ==  0 {
        101
    } else {
        ((1 as f64 /inp as f64) * 100 as f64) as i32
    }
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

    pub fn getId(&self) -> i32 {
        self.i
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

    pub fn get_weight(ride_a : &Ride, ride_b : &Ride) -> i32 {
        return invert(distance(ride_a, ride_b)) + time_difference(ride_a, ride_b) + ride_b.duration;
    }

    pub fn get_weight_given_time(current_time : i32, ride_a : &Ride, ride_b : &Ride) -> i32 {
        return invert(distance(ride_a, ride_b)) + time_difference_given_time(current_time, ride_a, ride_b) + ride_b.duration;
    }
}
