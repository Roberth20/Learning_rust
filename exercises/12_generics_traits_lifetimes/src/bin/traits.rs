// To use the trait, we must bring to scope the type and the trait
use generics_traits_lifetimes::{SocialPost, NewsArticle, Summary};

fn main() {
    let post = SocialPost{
        username: String::from("horse_ebook"),
        content: String::from("A very casual book to consider"),
        reply: false,
        repost: false
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        )
    };
    // Here, we call a method from the trait that was not implemented in NewsArticle
    println!("New article: {}", article.summarize_default());
    // Here, we call a default method from the trait which call another
    println!("1 new post: {}", post.summarize_default_with_call());

    // We can use the trait as parameter of functions
    fn notify(item: &impl Summary) {
        // This way, we can call the function with any type that implement the trait Summary
        println!("Breaking news! {}", item.summarize());
    }
    notify(&article);
    // But the standard way to implement trait as parameter is with generics, and can be
    // extended easely to various parameters
    fn notify2<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking big news! {}", item1.summarize());
        println!("Breaking news! {}", item2.summarize());
    } 
    // To specify more than one trait, we use +
    use std::fmt::{Display, Debug};
    fn notify3<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // With multiple traits bounds and generics its become harder to read the signature
    // to write everything clearer we do it this way
    // Intsead
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        1
    }
    // use the clause where
    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug
    {
        1
    }

}

// Traits also can be used as return signature, always as the return value
// is a unique type
fn return_summarizable() -> impl Summary {
    // This way with can return any value which implement the trait, mostly
    // useful with clousures and iterators.
    SocialPost {
        username:String::from("horse_ebook"),
        content: String::from("A very casual book to consider"),
        reply: false,
        repost: false
    }
}