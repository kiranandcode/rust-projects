use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
static NTHREADS: i32 = 3;
fn main() {

    let (tx, rx) : (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (strin, strout) : (Sender<String>, Receiver<String>) = mpsc::channel();

//    for id in 0..NTHREADS {
//        let thread_tx = tx.clone();
////        thread::spawn(move || {
//            thread_tx.send(id).unwrap();
//            println!("Thread {} finished", id);
//        });
//    }
//
//    let mut ids = Vec::with_capacity(NTHREADS as usize);
//    for _ in 0..NTHREADS {
//        ids.push(rx.recv());
//    }

//    println!("{:?}", ids);

    let thread_4 = thread::spawn(move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        let result = format!("{} + {} + {} = {}", x, y, z, x + y + z);
        strin.send(result);
    });

    let clone = tx.clone();
    let thread_1 = thread::spawn(move || {
        clone.send(2 * 10)
    });
    let clone = tx.clone();
    let thread_2 = thread::spawn(move || {
        clone.send(2 * 20);
    });
    let clone = tx.clone();
    let thread_3 = thread::spawn(move || {
        clone.send(30 + 40);
    });

    let string = strout.recv().unwrap();
    println!("{:?}", string);


}


