use std::{sync::mpsc, thread};
use std::sync::Mutex;
use std::sync::Arc; 

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
// a atype alias for a trait object that holds  a type of closure execute expects

enum Message{
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    ///Create a new thread pool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is zero
     pub fn new(size: usize) -> ThreadPool {
        assert!(size>0); //will panic if size of thread pool is less than or ewqual to zero

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers  = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); //we need shared ownership and mutability of receiver
        }

        ThreadPool {workers,sender}
     }

     pub fn execute<F>(&self, f: F)
     where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap(); //if all threads stop running this will panic
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers ");
        for worker in &mut self.workers {
            println!("Shutting down worker { }",worker.id);
            //worker.thread.join().unwrap();

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message  = receiver
                        .lock()
                        .unwrap()
                        .recv()
                        .unwrap();
            //println!("Worker {} got a job, executing it.",id);
            //job();
            match message {
                Message::NewJob(job) => {
                    println!("worker got a Job {}",id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate",id);
                    break;
                }
            }
        });
        Worker {id,thread: Some(thread)}
    }

}