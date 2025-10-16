/* For cases when a sinlge value might have multiple owners, like graph data structures
we use the reference counting Rc<T> which keeps track of the number of references to a 
value. This only work with single thread scenarios.

We will recover the cons list with boxes. With the architecture

b -> [3|-] 
           \
        a -> [5| -] -> [10| -] -> [Nil]    
           /
c -> [4|-]
*/
/*enum List {
    Cons(i32, Box<List>),
    Nil,
}*/
// Improving List to use Rc<T>
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    /* This fails because Cons in b take ownership and it's no valid anymore.
    We could change it by Cons holding references with lifetime, which can work only in 
    some scenarios*/
    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));

    let a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(Nil))
        ))
    );
    println!("Count after creating a = {}", Rc::strong_count(&a));
    // Here we create two new owners for the data that hold a. Making a, b and c the 
    // owners
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    // Rs::clone(&a) if different to a.clone() because does not make a full copy of the 
    // data, instead only incremet the reference counting
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}