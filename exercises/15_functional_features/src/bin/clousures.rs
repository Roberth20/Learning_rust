#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // This is a clousure with no parameters
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Example of how a function is similar to a clousure
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32|             { x + 1 };
    let add_one_v4 = |x: u32|               x + 1  ;

    // Inmutable borrow
    let list = vec![1, 2, 3];
    println!("Before defining clousure: {list:?}");

    let only_borrows = || println!("From clousure: {list:?}");

    println!("Before calling clousure: {list:?}");
    only_borrows();
    println!("After calling clousure: {list:?}");

    // Mutable borrow
    let mut list = vec![1, 2, 3];
    println!("Before defining clousure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling clousure: {list:?}");

    // Taking ownership, useful only to move ownership to a new thread of process
    let list = vec![1, 2, 3];
    println!("Before defining clousure: {list:?}");
    // Take ownership with move before the parameter
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    /* The clousure can move the captured value outside of it, mutate the value, 
    none of them and capture nothing. Depending of how the clousure capture and handle
    values, will be a some trait: FnOnce, FnMut, Fn.
    
    For example, unwrap_or_else.
    
    impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        // Here, the trait is FnOnce, so the clousure as minimum can be called once 
        // and without take parameters
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}*/

// Checking for example sort_by_key, which use FnMut
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let mut list = [
    Rectangle { width: 10, height: 1},
    Rectangle { width: 3, height: 5},
    Rectangle { width: 7, height: 12},
];
// Gets as argument a reference to current item and return the value to be ordered
list.sort_by_key(|r| r.width);
println!("{list:#?}");

    
}