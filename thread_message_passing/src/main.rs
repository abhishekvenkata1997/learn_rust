
use std::sync::mpsc; //multi producers single consumer of information
use std::{thread,time::Duration};

fn main() {
    //threads passing messages to each other containign data
    //do not communicate by sharing memory
    //instead share memory by communicating
    //use channels -> like channel of water
    // if we put something on the channel it will travel
    //it has transmitter receiver
    //transmitter = upstream location, receive = downstream location
    //one part sends information other part receives information
    //channel is open when both are available and doing their tasks
    //one side performing opersations other capturing results


    let (tx,rx) = mpsc::channel(); //get a tuple that has a sender(transmitter) and receiver

    //trying to pass multiple messages i.e vector of values and send each message
    /*let handle = thread::spawn(move || {
        let  msg: String = String::from("hi from transmitter");
        tx.send(msg).unwrap(); //send method returns result type, if receiver is not available this will return an error    
        //println!("msg is {}",msg); //this will give an error because msg ownership is changed
    });

    handle.join().unwrap();*/
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Thread"),
            String::from("transmitter")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //rx is treated as an iterator without any need of using the rx.recv() function
    
    for received in rx {
        println!("Got from thread: {}",received);
    }

    handle.join().unwrap();
    //let received = rx.recv().unwrap();
    //try_recv will return and not block thread
    //recv will block thread and wait for message 

    //println!("Got: {}",received);

}
