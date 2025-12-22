// NOTE: Function pointers
// Just as we pass clousures to functions, we can pass regular functions to
// functions. The function coerce to the type `fn` which is different from the
// clousure trait `Fn`. The `fn` type is called *function pointer* and using
// them is similar to passing clousures.
//
// Unlike clousures, `fn` is a type rather than a trait, so we need to specify
// `fn` as the parameter type directly. Also, this type implements the clousure
// traits (`Fn`, `FnMut`, `FnOnce`) meaning we always can pass functions to
// functions that expects a clousure.

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// NOTE: Returning clousures
// Clousures are represented by traits, which mean we can not return clousures
// directly. To return a trait, we do it with the type that implement it.
// However, clousures don't have a concrete type to return. Instead, we will use
// the `impl Trait` syntax to return a function type.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// The previous works when the the closure is also its own unique type. When the
// signature is the same for multiple functions but the closure is different. We
// must use a trait object. This happens because Rust creates unique *opaque* types
// that implements such traits.
fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    // Simple example
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    // Using clousure or function as parameter
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = { list_of_numbers.iter().map(|i| i.to_string()).collect() };
    let lis_of_strings2: Vec<String> =
        { list_of_numbers.iter().map(ToString::to_string).collect() };

    // We can use this method to initialize enums.
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // Returning closure
    let handlers = vec![returns_closure2(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}
