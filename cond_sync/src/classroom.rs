extern crate rand;
use std::sync::{Arc, Condvar, Mutex};
use std::time;
use std::thread;

use self::rand::Rng;


/* 
 * Classroom struct
 * 
 *
 */
pub struct ClassroomInternal {
    waiting_spaces: i16,
    free_computers: u32,
}

pub struct Classroom (Arc<(Mutex<ClassroomInternal>,Condvar)>);

impl Classroom {
    pub fn new() -> Self {
        Classroom(Arc::new((Mutex::new(ClassroomInternal {
            waiting_spaces: 5,
            free_computers: 25,
        }), Condvar::new())))
    }

    pub fn clone(&self) -> Self {
        Classroom(self.0.clone())
    }

    pub fn get_state(&self) -> (i16, u32) {
       let &(ref lock, ref monitor) = &*self.0;
       let mut classroom = lock.lock().unwrap();
       (classroom.waiting_spaces, classroom.free_computers)
    }

    pub fn enter(&self) -> bool {
       let &(ref lock, ref monitor) = &*self.0;
       let mut classroom = lock.lock().unwrap();
       if classroom.waiting_spaces == 0 && classroom.free_computers == 0 {
            false
       } else {
           classroom.waiting_spaces = classroom.waiting_spaces - 1;
           while classroom.free_computers == 0 { classroom = monitor.wait(classroom).unwrap(); }
           classroom.waiting_spaces = classroom.waiting_spaces + 1;
           classroom.free_computers = classroom.free_computers - 1;
           true
       }
    }


    pub fn exit(&self)  {
        let &(ref lock, ref monitor) = &*self.0;
        let mut classroom = lock.lock().unwrap();
        classroom.free_computers = classroom.free_computers + 1;
        monitor.notify_all();
    }
}


pub fn classroom_controller_example() {
    let classroom = Classroom::new();

    for i in 0..3 {
        let classroom_ = classroom.clone();
        thread::spawn(move || {
            let entrance = classroom_;
            let id = i;
            let mut rng = rand::thread_rng();
            loop {
                let time_to_wait = (rng.next_f32() * 30.0 * 1000.0) as u64;
                let time_millis = time::Duration::from_millis(time_to_wait);
                println!("Thread {}: entering the classroom", id);
                if !entrance.enter() {
                   println!("Thread {}: Could not enter the computer room", id); 
                } else {
                    thread::sleep(time_millis);
                    entrance.exit();
                    println!("Thread {}: exiting the classroom", id);
                }
            }
        });
    }


    let time_millis = time::Duration::from_millis(1000 * 10);
    loop {
            thread::sleep(time_millis);
            let (waiting, free) = classroom.get_state();
            println!("Classroom-state (waiting: {}, free: {})", waiting, free);
    }
}

