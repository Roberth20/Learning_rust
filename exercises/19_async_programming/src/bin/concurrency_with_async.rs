// As with threads, we can handle concurrency with async.
use std::time::Duration;

fn main() {
    // Create the async runtime
    trpl::run(async {
        // Internal or second threadthread
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        // main thread
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // If we want the internal task to keep running when main thread is off, we
        // have to wait it
        //handle.await.unwrap();
    });

    // The bigger difference between thread and async is we don't need to spawn a task, we can 
    // run them until completion by joining them
    trpl::run(async{
        let fut1 = async {
            for i in 1..10 {
                println!("Hi number {i} from thrid task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Hi number {i} from the fourth task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await;
    });

    // For transfer data, is similar to threads, only with async versions of functions 
    // and types. First, without spawning a task
    trpl::run(async {
        // async version of threads channel
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();

        // The async versión don't block, instead, give control to the runtime until the 
        // message is received of the channel is closed 
        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
    // Now, sending messages with some sleep between them
    trpl::run(async{
        let (tx, mut rx) = trpl::channel();
        // Both need to be in async blocks to run concurrent
        // We move ownership to the async block to close the channel when it ends
        // Also, as the channel allow multiple producer, we can clone the tx
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("incoming"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        // But this approach with join to wait all task to finish is not scalable
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}