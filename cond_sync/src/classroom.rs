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



