

//mutex = mutual exclusion
//only one thread can access data at a point of time
//signal that i want access, acquire mutex lock - use exclusive access 
//no other thread can access that data during locks
//then you unlock the data and other thread can use
//1. Acquire a lock before access to data
//2. Release lock before other thread can have access
//3. Rust type systems helps and makes locking and unlocking easier

use std::sync::{Arc,Mutex};
use std::thread;
use std::rc::Rc;

fn main() {

    //mutex is like refcell even though we dont have a mutable counter, it allows us to change the value inside the counter
    //Mutex comes with the risk of creating deadlocks
    let m = Mutex::new(5);
    //let counter = Rc::new(Mutex::new(0)); rc is used to duplicate the same counter and use it in several places
    //but here rc is not enough we need an atomic rc for this to work
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {

                    
        //let counter = Rc::clone(&counter); cannot use this because rc is not thread safe we need to use an atomic rc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter: {:#?}",counter);

    {
        //m.lock() is in a result so need to unwrap()
        //because if other owner has a lock has that, it returns an error
        let mut num = m.lock().unwrap();
        // this is a type of a smart pointer
        // implements drop trait when num goes out of scope lock is released

        *num = 6;


    }

    println!("m = {:#?}",m);
}
