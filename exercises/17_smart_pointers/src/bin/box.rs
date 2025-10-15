/*Box<T> allo to store data on the heap, remaining in the stack only the pointer to the
heap data. They are used often for:

1. When you have a type whose size can’t be known at compile time and you want to use a 
value of that type in a context that requires an exact size

2. When you have a large amount of data and you want to transfer ownership but ensure 
the data won’t be copied when you do so

3. When you want to own a value and you care only that it’s a type that implements a 
particular trait rather than being of a specific type
*/

fn main() {
    // here we store a i32 value on the heap
    let b = Box::new(5);
    // Here we access to data as owned on the stack, also when the box goes out of
    // cope it clears the data from the stack and heap
    println!("b = {b}");

    // For 1. Take the example of cons list in Lisp, they are recursive list similar
    /* to linked functions. For example, without box
    enum List {
        Cons(i32, List),
        Nil,
    }
    // Then
    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    
    This code fails becauses Rust can not determine the space to allocate in memory for
    the enum. Now, instead of use a possible infinite structure, we use Box to give Rust
    a pointer to the data, the pointer have constant size and can work properly*/
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, 
        Box::new(
            List::Cons(2, 
                Box::new(
                    List::Cons(3, 
                        Box::new(List::Nil)
                    )
                )
            )
        )
    );
    // We can use Deref to treat smart pointers like regular references
    // A normal reference
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // For Box
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // let's build our own pointer like box, but this don't store data in the heap
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // But the deference will no work, we must add the Deref trait
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            // Here, we define which be returned when calling the deference *
            &self.0
        }
    }
    // With functions and methods we have Implicit Deref Coercions, this converts a
    // reference to a type that implements Deref trait into a reference to another type.
    // An example is convert &String to &str
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));
    // with &m get the reference to MyBox<String>, &MyBox<String> which then conert
    hello(&m);

    // Similarly, we can use DerefMut trait to work with mutable references

    /*Finally, Rust does deref coercion with three cases:
    1. From &T to &U when T: Deref<Target=U>
    2. From &mut T to &mut U when T: DerefMut<Target=U>
    3. From &mut T to &U when T: Deref<Target=U> */

    /*The Drop Trait allow to control what to do when the value goes out of scope.
    For example, to print something */
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer{
        data: String::from("some stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // The Drop trait don't allow to call the method to free the resources early.
    // In such case, we must use the drop function from std::mem::drop which is included
    // in the prelude.
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}