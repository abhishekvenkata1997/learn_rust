use std::{thread,time::Duration};

//one to one threads = native threads, system threads, green threads
//OS gives APIs to create threads -> One to one mapping between OS thread and local thread

//local or green threads = M green threads that match to N operating threads

//one to one threads are in standard library
//to use local or green threads use crates or libraries

fn main() {
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
            //if a main thread and a thread spawned thread exist, program will run only as long as main thread
            //executes, then it stops execution, no waiting for spawned thread
        }
    });

    // handle.join().unwrap(); -> Writing it here means completing this thread before we get started with the process in main
    //there is no concurrency in the operations then

    for i in 1..5 {
        println!("hi number {} from the main thread! ",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); //blocks the thread until the thread inside handle completes running or terminates
    //spawn thread finishes execution after main thread is done
    //if a main thread

    let v =  vec![1,2,3];

    //moving ownership of the vector v using 'move' inside our thread closure to make sure v does 
    //not go out of scope as long as the thread runs
    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}",v);
    });
    handle.join().unwrap();
}