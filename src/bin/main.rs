use std::sync::mpsc;
use std::thread;

use simple_threadpool::ThreadPool;

fn main() {
    // Create a thread pool with 4 threads
    let pool = ThreadPool::new(4).expect("Failed to create ThreadPool");

    // Create a channel for communication between the main thread and the worker threads
    let (tx, rx) = mpsc::channel();

    // Spawn some jobs
    for i in 0..10 {
        let tx_clone = tx.clone();
        pool.spawn(move || {
            println!("Job {} started", i);
            // Simulate some work
            thread::sleep(std::time::Duration::from_secs(1));
            println!("Job {} completed", i);
            // Notify the main thread that the job is done
            tx_clone.send(i).expect("Failed to send message");
        });
    }

    // Wait for all jobs to complete
    for _ in 0..10 {
        rx.recv().expect("Failed to receive message");
    }

    // Gracefully shutdown the thread pool
    pool.shutdown();
    // The drop implementation of ThreadPool will wait for worker threads to finish

    println!("All jobs completed");
}
