// NOTE: Newtype pattern.
// We saw this in `advanced_traits.rs`, they also allow to statically enforce
// values are not confused, like changing their name by the unit. They can
// abstract some implementation details of a type by exposing a different API.
//
// NOTE: Type synonyms and aliases
// They give the ability to give an existing type another name. For example:

type Kilometers = i32;

// This way kilometers is an alias for `i32` and is going to be treated the
// same as values of type i32. The main use of aliases and synonyms is to reduce
// repetition in complex types
type Thunk = Box<dyn Fn() + Send + 'static>;

// NOTE: The never type
// This is a special type `!` which is empty and stands in functions when a
// function never return. This types of functionas are called diverging functions.
//
// Some examples of this type are `continue`, `panic!` or loops without arguments

// NOTE: Dynamically sized types and the `Sized` trait
// In general, Rust needs to know how much space to allocate for a value of a
// particular type. This leads to the concept of *dynamically sized types* or
// DST or unsized types.
//
// An example of DST is `str`, not `&str`. When storing text by a user, we can't
// know how long the string will be. That means we can't create a variable of
// type str. Rust needs to allocate the same amount of memory for every variable
// of the type str, so with different lenghts, both needs different storages.
//
// The solution to this is using slices, this way we can hold the address to
// memmory and the lenght that hold the value, all to access from a pointer.
// This last detail, allow to combine str with all kinds of pointers like Box<str>
// or Rc<str>. A particular DST as Traits, when we used them behind pointers like
// &dyn Trait or Box<dyn Trait>.
//
// To work with DST, Rust provides the `Sized` trait to determine whether or not
// a type's size is known at compile time. This is implemented for everything
// whose size is known at compile time. In addition, Rust adds a bound on Sized
// for every generic function.

fn main() {
    // Example type alias
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
