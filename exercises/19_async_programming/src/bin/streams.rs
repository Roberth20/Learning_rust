use std::time::Duration;

/*Streams are like asynchronous iterators. As they are similar, we can create a stream
from a iterator */
// To use next method with stream, we need to import the Trait StremExt, an extension
// trait to improve the base Stream trait
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    trpl::run(async{
        // For example, the new trait allow to filter
        let mut filtered = stream.filter(|value| value % 3 == 3 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
        println!("The value was: {value}");
    }
    });
    
    /*As streams are futures, we can mix them with other futures to do interesting things
    For example, a stream messages like a WebSocket or real-time communication protocol */
    use trpl::{ReceiverStream, Stream};
    use std::{pin::pin, time::Duration};
    trpl::run(async{
        // To make it different from older code, we add a timeout and items delay
        let mut messages = 
            pin!(get_messages().timeout(Duration::from_millis(200)));
        // Also, we can join streams.
        let intervals = get_intervals()
            // with throttle, we can limit the rate the function will be called
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        // Both streams must be the same type, and set it to stop after 20 items from the stream
        let merged = messages.merge(intervals).take(20);
        // Also, the stream must be piined amd mutable
        let mut stream = pin!(merged);
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            // Error handling
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            }
        }
    });
    ReceiverStream::new(rx)
}