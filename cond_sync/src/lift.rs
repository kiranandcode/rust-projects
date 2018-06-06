extern crate rand;

use std::sync::{Mutex, Condvar, Arc};
use std::time;
use std::thread;

use self::rand::Rng;

let no_floors : 'static usize = 10;

enum LiftDirection {
    UP, DOWN, NEITHER
}

struct Lift {
    going_up : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    going_down : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    waiting_out : Arc<(Mutex<[bool;no_floors]>, Condvar)>,
    lift_direction: Arc<Mutex<LiftDirection>>
}


impl Lift {

    pub fn new() -> Self {
        Lift {
            going_up: Arc::new(Mutex::new([0;no_floors], Condvar::new())),
            going_down: Arc::new(Mutex::new([0;no_floors], Condvar::new())),
            waiting_out: Arc::new(Mutex::new([false;no_floors], Condvar::new())),
            lift_direction: Arc::new(Mutex::new(LiftDirection::NEITHER)),
        }
    }

}
