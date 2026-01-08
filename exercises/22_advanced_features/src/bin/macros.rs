// NOTE: Macros are a way to write code that writes other code.
// For example, the `derive` attribute generates an implementation of some traits.
// `println!` and `vec!` are macros to expand and produce more code too.
//
// They have some similarities with functions to reduce the amount of code written,
// but where the functions must declare the parameters and types; the macros can
// take a variable number of parameters. Also, the macros are expanded before
// compiling while the functions are called at runtime. The downside of them is
// they are harder to write, understand and maintain, also they must define or
// bring to scope before call them in a file.

// NOTE: Declarative macros.
// They are the most widely used form of macros. They are like `match` expressions,
// comparing a value to patterns associated to with some code. To define a macro
// we use `macro_rules!`, for example, a simplified version of `vec!`
//
// `#[macro_export]` indicate that this macro should be available when the crate
// is brought into scope.
#[macro_export]
// We write the macro with name of it, without ! and using curly {} to define it.
macro_rules! vec {
    // The pattern syntax is diferent to values, so it's important to check
    // https://doc.rust-lang.org/reference/macros-by-example.html.
    //
    // For this, the () start the pattern, then with `$` we declare a variable
    // that contain the rust code matching pattern, then the inner capture inside
    // $(), match any Rust expression to the variable $x. The comma indicates each
    // instance must me separated with a comma and finally, * specifies that the
    // pattern matches zero or more of whatever precedes the *
    //
    // This way, when calling the macro vec![1, 2, 3], $x match three times with
    // the expressions 1, 2 and 3.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // Here, the $()* indicate that the code inside must be generated for
            // each match in $(), then $x is replaced by the expresion matched.
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// NOTE: Procedural macros to generate code from attributes
// They accept some code as input, operate on that code and produce some code.
// There three kinds of procedural macros being cusotm `derive`, attribute-like
// and function-like, all of them working similarly.
//
// They are created in their own crate with a especial crate type. They takes a
// `TokenStream` as input and output. This way, we can build any type of procedural
// macro.
//
// ```
// use proc_macro::TokenStream;
// Some macro variety
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}
// ```

// NOTE: Custom `derive` macros
// Let's practice the procedural macros by creating a derive macro. We will build
// a macro that annotate as #[derive(HelloMacro)] to get a default implementation
// of a hello_macro function. This default implementation will print a string with
// the variable `TypeName` where TypeName is the name of the type on which the
// trait has been defined.
//
// At current Rust version, the macros must reside in its own crate. Check the
// crate `hello_macro_derive` and this workspace `Cargo.toml`

// Create the trait
pub trait HelloMacro {
    fn hello_macro();
}

// Bring to scope the macro
use hello_macro_derive::HelloMacro;

// Build the struct with macro
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
