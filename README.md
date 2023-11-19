# simple-threadpool

A simple and efficient thread pool implementation in Rust.

## Features

- Ease of Use: Create a thread pool with a specified number of worker threads and seamlessly spawn parallelized jobs.
- Graceful Shutdown: Ensure a clean and graceful shutdown when the thread pool is no longer needed.
- Lightweight: Keep your codebase lightweight with a straightforward API designed for simplicity.

## Usage

An example can be found in the `examples` folder.

```rust
use simple_threadpool::{ThreadPool, Job};

fn main() {
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4).expect("Failed to create thread pool");

    // Spawn a job onto the thread pool
    pool.spawn(|| {
        // Your parallelized task goes here
        println!("Job executed in parallel!");
    });

}
```

## Improvements

- Graceful Shutdown Enhancement: Ensured that the thread pool can be gracefully shut down, preventing any lingering threads.
- Improved Error Handling: Incorporated more descriptive error messages for better debugging.
- Configurability: Allow the user to configure the thread pool size easily.

## License

This project is licensed under the MIT License.
