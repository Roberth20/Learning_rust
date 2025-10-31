/* To work with futures, we create a collection of them an iterate and join all of them. */

use core::time;
use std::process::Output;
use std::thread;
use std::time::Duration;
use std::pin::{Pin, pin};
use trpl::Either;

fn main() {
    trpl::run(async{
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        // We the futures are local to the function, we can Pin directly each future to
        // avoid heap allocation
        let tx1_fut = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });
        
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("incoming"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });
        // This won't work becasue each async block produce a Future trait.
        // let futures = vec![tx1_fut, rx_fut, tx_fut];
        /* To solve this, we use a trait object, like Box with correct type declaration.
        let futures: Vec<Box<dyn Future<Output = ()>>> = vec![
            Box::new(tx1_fut),
            Box::new(rx_fut),
            Box::new(tx_fut),
            ];
        trpl::join_all(futures).await;
        */
        // The previous fails because of Pin trait, to fix it.
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![
            tx1_fut,
            rx_fut,
            tx_fut,
        ];
        trpl::join_all(futures).await;
        // Join_all only works with futures of the same type, when they are different,
        // we use join!
        let a = async {1u32};
        let b = async {"hello!"};
        let c = async {true};

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });

    // Join is useful when we require all the futures. When we need some of them, we use
    // race
    trpl::run(async{
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished");
        };

        trpl::race(slow, fast).await;
    });

    // Now, if we have a long-running operation, how can we lay control back to runtime?
    fn slow(name: &str, ms: u64) {
        // Here, we use thread::sleep instead of trpl::sleep to simulate a real-world 
        // operation which is long-running anf blocking
        thread::sleep(Duration::from_nanos(ms));
        println!("'{name}' ran for {ms}ms");
    }
    trpl::run(async{
        // Togive control to runtime and execute other thing, we must signal something to
        // await. While a future is waiting for a value, process another
        let a = async {
            println!("'a' started.");
            // To await have something to await as fast as possible, we use yield_now
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
        trpl::race(a, b).await;
    });

    /* Own async abstractions. Timeout API.
    With async building blocks we can create more abstractions. For this example:
    1. I needs to be an async function
    2. The first parameters should be a future to run
    3. The second parameter will be the maximun time to wait
    4. It should return a Result. When Ok is the value from the future and Err the time
    waited for
    */
    async fn timeout<F: Future>(
        future_to_try: F,
        max_time: Duration,
    ) -> Result<F::Output, Duration> {
        // For this function, we have to race the future against a duration, remembering
        // that race is not fair (the first argument start executing first).
        match trpl::race(future_to_try, trpl::sleep(max_time)).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time),
        }
    }
    trpl::run(async{
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}