use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{
    Mutex,
    Arc
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}




fn thread_first_example() {

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
        println!("The vector is: {:?}", v);
    });


    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the main thread!",i);
        }
    });

    thread::sleep(Duration::from_secs(30));

    handle.join();
    handle2.join();
}

fn thread_second_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string()
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });


    for recieved in rx {
        println!("Got: {}", recieved);
    }

}


fn thread_third_example() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
         let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string()
        ];

         for val in vals {
             tx1.send(val).unwrap();
             thread::sleep(Duration::from_secs(1));
         }
    });

    thread::spawn(move || {
         let vals = vec![
            "more".to_string(),
            "messages".to_string(),
            "for".to_string(),
            "you".to_string()
        ];

         for val in vals {
             tx.send(val).unwrap();
             thread::sleep(Duration::from_secs(1));
         }
    });


    for recieved in rx {
        println!("Recieved: {}", recieved);
    }

}
