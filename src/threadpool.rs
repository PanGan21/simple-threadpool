use std::{
    sync::{Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
    shutdown_flag: Arc<Mutex<bool>>,
}

struct Worker(usize, Option<thread::JoinHandle<()>>);

impl ThreadPool {
    pub fn new(threads: u32) -> Result<Self, &'static str> {
        if threads == 0 {
            return Err("ThreadPool size must be greater than 0");
        }

        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let shutdown_flag = Arc::new(Mutex::new(false));

        let mut workers = Vec::with_capacity(threads as usize);

        for id in 0..threads {
            workers.push(Worker::new(
                id as usize,
                Arc::clone(&receiver),
                Arc::clone(&shutdown_flag),
            ));
        }

        Ok(ThreadPool {
            workers,
            sender,
            shutdown_flag,
        })
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if *self.shutdown_flag.lock().unwrap() {
            // Skip spawning new jobs during shutdown
            return;
        }

        let boxed_job = Box::new(job);
        self.sender.send(boxed_job).unwrap();
    }

    pub fn shutdown(&self) {
        *self.shutdown_flag.lock().unwrap() = true;
        // Notify workers by sending a dummy job to unblock them
        for _ in &self.workers {
            self.sender.send(Box::new(|| {})).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
        for worker in &mut self.workers {
            if let Some(thread) = worker.1.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>,
        shutdown_flag: Arc<Mutex<bool>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            if *shutdown_flag.lock().unwrap() {
                // Exit the loop and thread during shutdown
                break;
            }

            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker(id, Some(thread))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn create_thread_pool() {
        let pool = ThreadPool::new(4);
        assert!(pool.is_ok());
    }

    #[test]
    fn spawn_jobs() {
        let pool = ThreadPool::new(2).expect("Failed to create ThreadPool");
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.spawn(move || {
                let mut counter = counter_clone.lock().unwrap();
                *counter += 1;
            });
        }

        // Sleep to ensure all jobs have a chance to complete
        thread::sleep(std::time::Duration::from_secs(2));

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn drop_thread_pool() {
        let _pool = ThreadPool::new(3).expect("Failed to create ThreadPool");

        // No assertion, just testing that it doesn't panic on drop
    }
}
