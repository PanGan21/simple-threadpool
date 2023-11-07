use std::thread;

use simple_threadpool::ThreadPool;

fn main() {
    // Create a new ThreadPool with 4 threads
    let pool = ThreadPool::new(4).expect("Failed to create ThreadPool");

    // Spawn some jobs
    for i in 0..8 {
        let job = move || {
            println!("Job {} started by thread {:?}", i, thread::current().id());
            thread::sleep(std::time::Duration::from_secs(1));
            println!("Job {} completed by thread {:?}", i, thread::current().id());
        };

        pool.spawn(job);
    }

    // Sleep for a while to allow the threads to finish their jobs
    thread::sleep(std::time::Duration::from_secs(5));
}
