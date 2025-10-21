/*Async programming work with the concept of *futures*, async and await methods.
Futures is a trait that represent values that are not ready but they will become available
at some point.

With async keyword we define functions or block that can be interrupted and resumed, and
the await keyword allow to wait for the future to be ready.

We will work with trpl which re-exports items from *future* and *tokio* crates to create
a simple web scraper*/

use std::future::Future;
use trpl::{Either, Html};

// Return the text of web page title
async fn page_title(url: &str) -> (&str, Option<String>) {
    // Those are futures, in Rust they are lazy. They don't do anything until request them
    // with await keyword
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    // Equivalent to
    // let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html()
    );
    (url, title)

}

// Writing the previous function is equivalent to write a normal function which return a
// future of the return type
fn page_title_(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        // Each time we call await, it represent an await point which track the state 
        // involved to recover in runtime
        let text = trpl::get(url).await.text().await;
    Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // The async code needs a runtime, which normally comes the crates.
    trpl::run(async {
        // only builds the page_title getters, they do nothing until call
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        // race return Either type, is like result but the two options are left or right
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}