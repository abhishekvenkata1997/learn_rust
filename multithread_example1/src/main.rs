
use std::sync::mpsc;
use std::{thread,time::Duration};

fn main(){

    let (tx,rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Thread 1"),
            String::from("#1 Hi"),
            String::from("#1 To"),
            String::from("#1The"),
            String::from("#1 Receiver")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Thread 2"),
            String::from("#2 Hi"),
            String::from("#2 To"),
            String::from("#2 The"),
            String::from("#2 Receiver")
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for receiver in rx {
        println!("Got: {}",receiver);
    }
}