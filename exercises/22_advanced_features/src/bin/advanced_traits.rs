// NOTE: Associated types
// Their connect a type placeholder with the trait such the method definitions
// can use the placeholder in their signatures. This way, when implementing the
// trait, the type is define.
//
// For example, the `Iterator` trait with the associated type `Item`
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// They look similar like generics
struct Counter(usize);

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
// An the iterator with generics
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

// The difference between generics and associated types, is with the first we
// must annotate the type for each implementation, leading to repetitive code.

// NOTE: Default generic parameters and operator overloading
// When using generics, we can specify a default concrete type for the generic.
// This eliminate the need to implement the for such especific type. The syntax
// is <PlaceholderType=ConcreteType>.
//
// This is useful for operator overloading, where we can customize the behavior
// of an operator in some situations. Rust doesn't allow to create operators
// or overload arbitrary operators, we only can overload operations and traits
// from `std::ops`
use std::ops::Add;

// Here, Add trait have this definition
// trait Add<Rhs=Self> {
//    type Output;
//
//    fn add(self, rhs: Rhs) -> Self::Output;
//}
// Where Rhs=Self is the default parameter definition, and Rhs stands for right
// hand side. This means, the if we don't implement the parameter, the default
// type for rhs is Self.

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
// Example with default type definition
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
struct Milimeters(u32);
struct Meters(u32);
// Example changing default parameter, changing how add behavior when trying to
// add Milimeters plus Meters.
impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Meters) -> Self::Output {
        Milimeters(self.0 + (other.0 * 1000))
    }
}

// We use default type parameters in two ways:
// 1. To extend a type without breaking existing code.
// 2. To allow customization in specific cases most users won't need.

// NOTE: Disambiguating between identically named methods.
// Nothing in Rust prevents traits from having methods with the same name as
// another method. Because of this, when calling methods with the same name,
// we must tell which one we want to use.
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// NOTE: Supertraits
// When writing a trait definition that depends on another trait, the second one
// is called *supertrait*. This usually happens when the type to implement in
// the first trait requieres to immplement the second trait.
//
// For example, making an `OutlinePrint` trait with an `outline_print` method
// to print a value framed in astericks. So, given a `Point` struct that
// implements the `Display` trait. So, when implementing `outline_print` method,
// we want to use `Display` trait's functionality, because ot this, `OutlinePrint`
// will only work for types which implements Display and provide the needed
// functionalities.
use std::fmt;

trait OutlinePrint: fmt::Display {
    // The syntas is similar to trait bounds
    fn outline_print(&self) {
        // this method is implemented in any type that implements Display.
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// Creating a type which implement `to_string` method from `Display` trait.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

// NOTE: External traits with Newtype pattern.
// As learned way before, the orphan rule doesn't allow to implement a trait on
// type if at least one of them is not local to our crate. We can go around this
// rule by creating a new type in a tupe struct with one field, this works as a
// wrapper to implement the trait.
//
// It's import to note there is no runtime performance penalty when doing it.
// For example, implementing Display in Vec<T>.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Validation of add Points
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // When calling methods with same name, the syntax is `Trait::method`. This
    // only works because of the self parameter to help figure out the type of
    // self.
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    // When calling methods without self parameters, the syntax is
    // `<Type as Trait>::function(params)`.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!("The name of our baby dog is {}", Dog::baby_name());

    // Outline a Point with astericks
    let point = Point { x: 1, y: 2 };
    point.outline_print();

    // Display a Vec with a wrapper
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
