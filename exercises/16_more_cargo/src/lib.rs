//! Also, we have this type of comments, that document the item which *contains*  the 
//! comment (`lib.rs`) instead of the item *following* the comment, like below.
//! 
//!  # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

pub mod art;

/// Adds one to the number given.
///
/// # Examples
/// Here, the examples have two functionalities, give to the reader an idea of the code
/// and also works as a test with `cargo test`
///
/// ```
/// let arg = 5;
/// let answer = more_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// Some common sections are
/// 
/// # Panics
/// 
/// The scenarios in which the function being documented could panic. Callers of the 
/// function who don’t want their programs to panic should make sure they don’t call 
/// the function in these situations.
/// 
/// # Errors
/// 
/// If the function returns a Result, describing the kinds of errors that might occur 
/// and what conditions might cause those errors to be returned can be helpful to 
/// callers so they can write code to handle the different kinds of errors in different 
/// ways.
/// 
/// # Safety 
/// 
/// If the function is unsafe to call (we discuss unsafety in Chapter 20), there should
/// be a section explaining why the function is unsafe and covering the invariants that 
/// the function expects callers to uphold.


pub fn add_one(x: i32) -> i32 {
    x + 1
}