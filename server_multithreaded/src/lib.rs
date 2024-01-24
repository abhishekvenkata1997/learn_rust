use std::{sync::mpsc, thread};
use std::sync::Mutex;
use std::sync::Arc; 

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
// a atype alias for a trait object that holds  a type of closure execute expects
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
        self.sender.send(job).unwrap(); //if all threads stop running this will panic
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job  = receiver
                        .lock()
                        .unwrap()
                        .recv()
                        .unwrap();
            println!("Worker {} got a job, executing it.",id);
            job();
        });
        Worker {id,thread}
    }

}