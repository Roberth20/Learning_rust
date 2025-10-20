/* To communicate the theads we use a channel. One part of your code calls methods on 
the transmitter with the data you want to send, and another part checks the receiving 
end for arriving messages. A channel is said to be closed if either the transmitter or 
receiver half is dropped.*/

use std::sync::mpsc; // mpsc stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    // We create a channel, with channel function which return a transmiter and a receiver
    let (tx, rx) = mpsc::channel();
    // Here, we build a thread that sends a message. With move, tx is owned by the the thread
    thread::spawn(move || {
        let val = String::from("hi");
        // When sending a value, the ownership is transfered
        tx.send(val).unwrap();
    });
    /* Now, here is the receiver. We have two methods, recv and try_recv, with recv we 
    block the main thread until receive a value from the channel, if the transmitter
    closes, it will return a error. With try_recv we don't block the processs,  instead 
    return a Result<T, E> immediately: an Ok value holding a message if one is 
    available and an Err value if there aren’t any messages this time.
    */
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // A more detailed thread messaging
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // Here, the receiver can be treated as an iterator too. When the channel is closed,
    // the iteration end.
    for received in rx {
        println!("Got: {received}");
    }

    // Finally, as mpsc, can have multiple transmmiters, let's build more of them by cloning.
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("other"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("Got mp: {received}");
    }
}