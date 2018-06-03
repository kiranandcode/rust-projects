extern crate rand;
use std::sync::{Arc, Condvar, Mutex};
use std::time;
use std::thread;

use rand::Rng;


struct Carpark {
    car_lock : Arc<(Mutex<u32>,Condvar)>,
}


impl Carpark {

    pub fn new() -> Self {
        Carpark {
            car_lock: Arc::new((Mutex::new(0),Condvar::new()))
        }
    }

    pub fn clone(&self) -> Self {
        Carpark {
            car_lock:  Arc::clone(&self.car_lock)
        }
    }

    pub fn enter(&self) {
        println!("Entering carpark");
       let &(ref lock, ref monitor) = &*self.car_lock;
       let mut count = lock.lock().unwrap();
       *count = *count + 1;
       monitor.notify_all();
    }

    pub fn leave(&self) {
        println!("Leaving carpark");
            let &(ref lock, ref monitor) = &*self.car_lock;
            let mut count = lock.lock().unwrap();
            while *count == 0 { count = monitor.wait(count).unwrap();}
            *count = *count - 1;
    }

    pub fn get_count(&self) -> u32 {
        let &(ref lock, ref monitor) = &*self.car_lock;
            let mut count = lock.lock().unwrap();
            return *count;
    }
}

fn main() {
    let carpark = Carpark::new();

    let carpark_ = carpark.clone();
    thread::spawn(move || {
        let entrance = carpark_;
        let mut rng = rand::thread_rng();
        loop {
            let time_to_wait = (rng.next_f32() * 30.0 * 1000.0) as u64;
            let time_millis = time::Duration::from_millis(time_to_wait);
            thread::sleep(time_millis);
            entrance.enter();
        }
    });

    let carpark_ = carpark.clone();
    thread::spawn(move || {
        let entrance = carpark_;
        let mut rng = rand::thread_rng();
        loop {
            let time_to_wait = (rng.next_f32() *  30.0 * 1000.0) as u64;
            let time_millis = time::Duration::from_millis(time_to_wait);
            thread::sleep(time_millis);
            entrance.leave();
        }
    });


    let time_millis = time::Duration::from_millis(1000 * 10);
    loop {
            thread::sleep(time_millis);
            println!("Car count: {}", carpark.get_count());

    }


}
