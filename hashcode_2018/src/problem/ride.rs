
pub struct Ride {
    start_X : i32,
    start_Y : i32,
    end_X : i32,
    end_Y : i32,
    earliest_start : i32,
    latest_end : i32,
    duration : i32,
    
    latest_start : i32,
    earliest_end : i32
}


impl Ride {
    pub fn new(
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
            earliest_end: earliest_start + duration
        }
    }

}
