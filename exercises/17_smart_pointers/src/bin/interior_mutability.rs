/*This is a design pattern to mutate data when there are inmutable references to it.
As this feature is not allowed by the borrowing rules, is a unsafe code. 

We can explore the concept with RefCell<T>*, it's similar to Box<T> but instead of
enforce the rules at compile time, they are enforced at runtime. Because of this, when 
breaking rules we will get a compiler error. With RefCell<T>, the program will panic if 
rules are broken. 

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios. A common 
scenario for Interior Mutability is Mock Objects, by default Rust doesn't hace objects,
or mock objects funcionality. However, we can create a struct that serve the same purpose
*/

use std::{cell::{RefCell}, rc::Rc};

// Here, the Messenger Trait will be the interface for our mock object
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger, {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { 
            messenger, 
            value: 0, 
            max}
        }
    // As set_value doesn't return anything, we need to implement messenger trait to test
    // the behavior for different values
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've use up over 75% of your quota!");
        }
    }
}

/* Another use for RefCell, beside mocking objects is with Rc<T> to allo multiple owners 
of mutable data. Taking the cons list from the other examples, we can use RefCell<T> to 
modify value stored the lists*/
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

#[cfg(test)]
mod tests {
    // In this case, instead of edit the trait, we can store sent_messages within 
    // RefCell<T>
    use super::*;
    // Edit for RefCell
    use std::cell::RefCell;

    struct MockMessenger {
        // old
        //sent_messages: Vec<String>,
        // new
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // old
                //sent_messages: vec![],
                // new
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Here, it fails because send require self as a inmutable reference, but hence
            // sent_message change the value of MockMessenger, we get a compiler error.
            //self.sent_messages.push(String::from(message));
            // new
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // old
        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        // new
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    /*RefCell with the methods borrow() adn borrow_mut() work as inmutable and mutable 
    references, that follow the same ownership rules but enforced at runtime */

}