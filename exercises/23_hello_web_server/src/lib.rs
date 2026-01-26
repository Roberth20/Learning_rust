use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// First we create the structure
pub struct ThreadPool {
    // When we create a thread, usually expects the code to run as soon is created.
    // So, we need the create them and then wait for the code. To do this, we
    // create an intermediate class Worker.
    workers: Vec<Worker>,
    // Create the sender holder to communicate with the workers
    sender: Option<mpsc::Sender<Job>>,
}

// Second, we create the new method.
impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The ize is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // Create the sender and receiver
        let (sender, receiver) = mpsc::channel();

        // As the consumer is unique, and we need it for multiple consumers, we
        // can fix this by using Arc to let multiple workers own the receiver and
        // Mutex to ensure only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        // Then, create the threads. We use with_capacity as we now the number of
        // items in the vector, this give us a little more of performance.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // Third, add an execute method that takes as input a clousure.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Here we create the jon and send it to the workers
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Now, we add a Drop trait to the ThreadPool, to safely close all threads when
// the pool in dropped.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // First, drop the sender to stop possible jobs to send
        drop(self.sender.take());
        // to take ownership of the threads in the workers, we use Vec::drain method
        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

// Create the job type which hold the code or the clousure. For this, we create
// a trait object to hld the clousure
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // We create the worker with a empty clousure, which loop forever
        // asking for jobs
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                // If the channel is closed
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    // Break the loop to avoid listening the closed channel
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
