/* Shared-State concurrency is not recommended. Channels are similar to single ownership,
 shared-memory is like multiple ownership, adding complexity to code and systems.

We can do this with 'Mutex' an abbreviation fon mutual exclusion, it allows one thread 
to access some data at any given time by acquiring a lock. The lock is a data structure 
that keeps track of the current exclusive access to the data. Mutexes follows two rules:

1. You must attempt to acquire the lock before using the data.
2. When you’re done with the data that the mutex guards, you must unlock the data so 
   other threads can acquire the lock.
 */

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Here we create the mutex with some data
    let m = Mutex::new(5);

    {   
        // Acquire the lock and the data as MutexFuard, wrapped in LockResult
        let mut num = m.lock().unwrap();
        *num = 6;
        // When MutexGuard goes out of scope, it release the lock automatically
    }

    println!("m = {m:?}");

    // Between multiple threads, we share the ownership of Mutex with Arc<T>, concurrent
    // version of RC.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}