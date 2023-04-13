use std::io::Error;
use std::{
    sync::{
        mpsc::{self, SendError},
        Arc, Mutex,
    },
    thread,
};

/// A ThreadPool represented as workers which instantiate new threads to execute clojured provided by the sender as Jobs
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            if let Ok(worker) = Worker::new(id, Arc::clone(&receiver)) {
                workers.push(worker);
            }
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Takes a clojure and executes it using workers from the ThreadPool
    pub fn execute<F>(&self, f: F) -> Result<(), SendError<Box<(dyn FnOnce() + Send + 'static)>>>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(sender) = self.sender.as_ref() {
            sender.send(job)?;

            Ok(())
        } else {
            println!("Failed to send Job. Sender has been dropped.");

            Ok(())
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect("Failed to call join on worker thread while dropping");
            }
        }
    }
}

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Error> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Ok(Worker {
            _id: id,
            thread: Some(thread?),
        })
    }
}
