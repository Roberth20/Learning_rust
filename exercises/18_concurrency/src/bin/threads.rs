/*The Rust standard library uses a 1:1 model of thread implementation, whereby a 
program uses one operating system thread per one language thread.*/

use std::thread;
use std::time::Duration;

fn main() {
    // We use threads with spawn() with the code inside a clousure.
    /*thread::spawn(||
        for i in 1..10 {
            println!("Hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    );*/
    // If we want to keep the value and wait for the result
    let handle = thread::spawn(||
        for i in 1..10 {
            println!("Hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    );
    
    // If we put the join before the for loop, the main thread will wait before continue.
    // So, where the join is called affects if the threads run simultaneosly.
    // handle.join().unwrap()

    // The process (and threads) wil stop when the main thread finish its process, 
    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // With the owned value JoinHandle<T> and the join method,we can wait the thread
    handle.join().unwrap();

    // With 'move' we can give ownership of values to the clousures inside threads
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}