/*A trait defines the functionality a particular type has and can share with other 
types. We can use traits to define shared behavior in an abstract way. We can 
use trait bounds to specify that a generic type can be any type that 
has certain behavior. The behaviors are methods for such type*/

// First, we start creating the trait 
pub trait Summary {
    // Inside the trait we declare the method signature that will be shared
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // The method can have a default behavior if the implementation don't provide one
    fn summarize_default(&self) -> String {
        String::from("(Read more..)")
    }
    // Default implementation can call other methods of trait too
    fn summarize_default_with_call(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// This way the compiler will enfore all behaviors, with their own implementations,
// to have the same signature.

// Now, we implement the trait in different methods and structures
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
// Implement trait following this pattern
impl Summary for NewsArticle {
    // The adding the method and signature of the trait with the behavior for this type 
    // (the struct NewsArticle)
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool 
}
// Implement trait
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// With traits bounds, we can implement methods conditioned by the traits of the type
use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T
}
// General method
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x,y}
    }
}
// Mehtod only if the type T implement the traits Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        // PartialOrd, logic comparation
        if self.x >= self.y {
            // Display trait, can be printed.
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

/* We can also conditionally implement a trait for any type that implements another
trait. Implementations of a trait on any type that satisfies the trait bounds 
are called blanket implementations An example is ToString Trait

impl<T: Display> ToString for T {
    // something
}
*/