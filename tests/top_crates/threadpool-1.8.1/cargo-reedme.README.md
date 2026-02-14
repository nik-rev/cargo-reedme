A thread pool used to execute functions in parallel.

Spawns a specified number of worker threads and replenishes the pool if any worker threads
panic.

# Examples

## Synchronized with a channel

Every thread sends one message over the channel, which then is collected with the `take()`.

```rust
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

let n_workers = 4;
let n_jobs = 8;
let pool = ThreadPool::new(n_workers);

let (tx, rx) = channel();
for _ in 0..n_jobs {
    let tx = tx.clone();
    pool.execute(move|| {
        tx.send(1).expect("channel will be there waiting for the pool");
    });
}

assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
```

## Synchronized with a barrier

Keep in mind, if a barrier synchronizes more jobs than you have workers in the pool,
you will end up with a [deadlock](https://en.wikipedia.org/wiki/Deadlock)
at the barrier which is [not considered unsafe](
https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html).

```rust
use threadpool::ThreadPool;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};

// create at least as many workers as jobs or you will deadlock yourself
let n_workers = 42;
let n_jobs = 23;
let pool = ThreadPool::new(n_workers);
let an_atomic = Arc::new(AtomicUsize::new(0));

assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

// create a barrier that waits for all jobs plus the starter thread
let barrier = Arc::new(Barrier::new(n_jobs + 1));
for _ in 0..n_jobs {
    let barrier = barrier.clone();
    let an_atomic = an_atomic.clone();

    pool.execute(move|| {
        // do the heavy work
        an_atomic.fetch_add(1, Ordering::Relaxed);

        // then wait for the other threads
        barrier.wait();
    });
}

// wait for the threads to finish the work
barrier.wait();
assert_eq!(an_atomic.load(Ordering::SeqCst), /* n_jobs = */ 23);
```