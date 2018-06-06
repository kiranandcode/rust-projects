extern crate rand;

use std::sync::{Mutex, Condvar, Arc};
use std::time;
use std::thread;
use std::cmp::PartialEq;

use self::rand::Rng;

const no_floors : usize = 10;


#[derive(PartialEq)]
enum LiftDirection {
    UP, DOWN, NEITHER
}

pub struct Lift { going_up : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    going_down : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    waiting_out : Arc<(Mutex<[bool;no_floors]>, Condvar)>,
    lift_state: Arc<(Mutex<(bool,usize,LiftDirection)>, Condvar)>
}


impl Lift {

    pub fn new() -> Self {
        Lift {
            going_up: Arc::new((Mutex::new([0;no_floors]), Condvar::new())),
            going_down: Arc::new((Mutex::new([0;no_floors]), Condvar::new())),
            waiting_out: Arc::new((Mutex::new([false;no_floors]), Condvar::new())),
            lift_state: Arc::new((Mutex::new((false, 0, LiftDirection::NEITHER)), Condvar::new()))
        }
    }


    pub fn clone(&self) -> Self {
         Lift {
            going_up: self.going_up.clone(),
            going_down: self.going_down.clone(), 
            waiting_out: self.waiting_out.clone(), 
            lift_state: self.lift_state.clone()
        }
    }


    pub fn press_up_button(&self, from_floor: usize, to_floor: usize) {
        {
            let &(ref lock, _) = &*self.going_up;
            let mut going_up = lock.lock().unwrap();
            (*going_up)[from_floor] = (*going_up)[from_floor] + 1;
        }

        let &(ref lock, ref monitor) = &*self.lift_state;
        let mut state = lock.lock().unwrap();
        while !state.0 || state.1 != from_floor || state.2 != LiftDirection::UP {
            state = monitor.wait(state).unwrap();
        }
        monitor.notify_all();

        {
            let &(ref lock, _) = &*self.going_up;
            let mut going_up = lock.lock().unwrap();
            (*going_up)[from_floor] = (*going_up)[from_floor] - 1;
        }

        {
            let &(ref lock, _) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = true;
        }
    }

    pub fn press_down_button(&self, from_floor: usize, to_floor: usize) {
        {
            let &(ref lock, _) = &*self.going_down;
            let mut going_down = lock.lock().unwrap();
            (*going_down)[from_floor] = (*going_down)[from_floor] + 1;
        }

        let &(ref lock, ref monitor) = &*self.lift_state;
        let mut lift_state = lock.lock().unwrap();
        while !lift_state.0 || lift_state.1 != from_floor || lift_state.2 != LiftDirection::DOWN {
            lift_state = monitor.wait(lift_state).unwrap();
        }
        monitor.notify_all();

        {
            let &(ref lock, _) = &*self.going_down;
            let mut going_down = lock.lock().unwrap();
            (*going_down)[from_floor] = (*going_down)[from_floor] - 1;
        }

        {
            let &(ref lock, _) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = true;
        }
    }


    pub fn lift_at_floor(&self, current_floor: usize, lift_direction: LiftDirection) {
       // set the internal state 
    }

    pub fn doors_open(&self) {
        // open doors and wait for all enter or exits to finish
    }


    pub fn doors_close(&self) {
        // close doors update state
    }
}
