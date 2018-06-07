extern crate rand;

use std::sync::{Mutex, Condvar, Arc};
use std::time;
use std::thread;
use std::cmp::PartialEq;

use self::rand::Rng;

const no_floors : usize = 10;


#[derive(PartialEq,Clone,Copy)]
pub enum LiftDirection {
    UP, DOWN, NEITHER
}

pub struct Lift { going_up : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    going_down : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    waiting_out : Arc<(Mutex<[u32;no_floors]>, Condvar)>,
    lift_state: Arc<(Mutex<(bool,usize,LiftDirection)>, Condvar)>
}


impl Lift {

    pub fn new() -> Self {
        Lift {
            going_up: Arc::new((Mutex::new([0;no_floors]), Condvar::new())),
            going_down: Arc::new((Mutex::new([0;no_floors]), Condvar::new())),
            waiting_out: Arc::new((Mutex::new([0;no_floors]), Condvar::new())),
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

        // first, increment the number of people waiting to go up at the from floor
        {
            let &(ref lock, _) = &*self.going_up;
            let mut going_up = lock.lock().unwrap();
            (*going_up)[from_floor] = (*going_up)[from_floor] + 1;
        }

        // second, wait until the lift is moving in the right direction, at the right floor, and is
        // open
        {
            let &(ref lock, ref monitor) = &*self.lift_state;
            let mut state = lock.lock().unwrap();
            while !state.0 || state.1 != from_floor || state.2 != LiftDirection::UP {
                state = monitor.wait(state).unwrap();
            }
        }

        // then, decrement the number of people waiting to go up from the from floor
        {
            let &(ref lock, ref monitor) = &*self.going_up;
            let mut going_up = lock.lock().unwrap();
            (*going_up)[from_floor] = (*going_up)[from_floor] - 1;

            // need to notify all, as the lift thread is waiting for the number of people at a
            // floor to be 0
            monitor.notify_all();
        }


        // then, tell the lift that there is one more person waiting to get out at the to floor
        {
            let &(ref lock, _) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = waiting_out[to_floor] + 1;

            // no need to notify as noone cares about the existance of people at a floor
        }


        // wait until the lift is at the correct floor and the doors are open
        {
            let &(ref lock, ref monitor) = &*self.lift_state;
            let mut state = lock.lock().unwrap();
            while !state.0 || state.1 != to_floor {
                state = monitor.wait(state).unwrap();
            }
        }


        // tell the lift that there is one fewer person waiting ot get out at the to floor
        {
            let &(ref lock, ref monitor) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = waiting_out[to_floor] - 1;

            // need to notify as the lift thread is waiting for all people to get off.
            monitor.notify_all();
        }



    }

    pub fn press_down_button(&self, from_floor: usize, to_floor: usize) {

        // first, increment the number of people waiting to go down at the floor
        {
            let &(ref lock, _) = &*self.going_down;
            let mut going_down = lock.lock().unwrap();
            (*going_down)[from_floor] = (*going_down)[from_floor] + 1;
            // no need to notify, as no threads are waiting for people to exist at any floors
        }

        // second, wait until the lift is moving in the right direction, has the right floor, and
        // is open
        {
            let &(ref lock, ref monitor) = &*self.lift_state;
            let mut lift_state = lock.lock().unwrap();
            while !lift_state.0 || lift_state.1 != from_floor || lift_state.2 != LiftDirection::DOWN {
                lift_state = monitor.wait(lift_state).unwrap();
            }
        }

        // then, decrement the number of people who are waiting to go down at the floor
        {
            let &(ref lock, ref monitor) = &*self.going_down;
            let mut going_down = lock.lock().unwrap();
            (*going_down)[from_floor] = (*going_down)[from_floor] - 1;
            
            // we need to notify as the lift thread is waiting for all people at a floor to be 0
            monitor.notify_all();
        }

        // then tell the lift that there is one more person waiting to leave at the to floor
        {
            let &(ref lock, _) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = waiting_out[to_floor] + 1;
        }

        // wait for the lift to be at the right floor with the doors open
        {
            let &(ref lock, ref monitor) = &*self.lift_state;
            let mut lift_state = lock.lock().unwrap();
            while !lift_state.0 || lift_state.1 != to_floor {
                lift_state = monitor.wait(lift_state).unwrap();
            }
        }

        // tell the lift that there is one less person waiting to leave at the to floor
        {
            let &(ref lock, ref monitor) = &*self.waiting_out;
            let mut waiting_out = lock.lock().unwrap();
            waiting_out[to_floor] = waiting_out[to_floor] - 1;

            // need to notify as the lift thread is waiting for other threads to relinquish the
            // lock
            monitor.notify_all();
        }



    }


    pub fn lift_at_floor(&self, current_floor: usize, lift_direction: LiftDirection) {
       // set the internal state 
       let &(ref lock, _) = &*self.lift_state;
       let mut lift_state = lock.lock().unwrap();
       lift_state.1 = current_floor;
       lift_state.2 = lift_direction;
    }

    pub fn doors_open(&self) {
        // open doors and wait for all enter or exits to finish
        
        // first find out the direction and floor of the lift and record the doors as open
        let mut direction : LiftDirection = LiftDirection::NEITHER;
        let mut current_floor = 0;
        {
            let &(ref lock, ref monitor) = &*self.lift_state;
            let mut lift_state = lock.lock().unwrap();
            lift_state.0 = true;
            direction = lift_state.2;
            current_floor = lift_state.1;

            // need to notify as there are threads waiting for the door to be open
            monitor.notify_all();
        }

        match(direction) {
            LiftDirection::UP => {
                let &(ref lock_going_up, ref monitor_going_up) = &*self.going_up;
                let &(ref lock_waiting_out, ref monitor_waiting_out) = &*self.waiting_out;
                let mut going_up = lock_going_up.lock().unwrap();
                let mut waiting_out = lock_waiting_out.lock().unwrap();
                

                while going_up[current_floor] > 0 || waiting_out[current_floor] > 0 {
                    going_up = monitor_going_up.wait(going_up).unwrap();
                    waiting_out = monitor_waiting_out.wait(waiting_out).unwrap();
                }
                // nothing to do, all people have left, the doors open method can now close.
            },
            LiftDirection::DOWN => {
                let &(ref lock_going_down, ref monitor_going_down) = &*self.going_down;
                let &(ref lock_waiting_out, ref monitor_waiting_out) = &*self.waiting_out;
                let mut going_down = lock_going_down.lock().unwrap();
                let mut waiting_out = lock_waiting_out.lock().unwrap();
                

                while going_down[current_floor] > 0 || waiting_out[current_floor] > 0 {
                    going_down = monitor_going_down.wait(going_down).unwrap();
                    waiting_out = monitor_waiting_out.wait(waiting_out).unwrap();
                }
                // nothing to do, all people have left, the doors open method can now close.
            }
            LiftDirection::NEITHER => {
                println!("Doors open called with no direction");
                return;
            }
        }
 
    }


    pub fn doors_close(&self) {
        // close doors update state
        let &(ref lock, ref monitor) = &*self.lift_state;
        let mut lift_state = lock.lock().unwrap();
        lift_state.0 = false;
    }
}
