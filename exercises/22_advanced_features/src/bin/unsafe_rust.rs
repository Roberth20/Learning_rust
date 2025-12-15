// Unsafe Rust is like a *second language* that does not enforce memory safety.
// These capability allow to write code without the rejections of the compiler,
// but it can lead to problems if it not manage properly.
//
// Also, it is useful to work closely to hardware for low-level system programing
// or even to write operating system.
//
// To switch to unsafe rust, we only need to use the keyword `unsafe` and start
// the unsafe block. The actions that can be performed while unsafe are:
// * Deference a raw pointer
// * Call an unsafe function or method
// * Access or modify a mutable static variable
// * Implemet an unsafe trait
// * Access fields of `union` S.
//
// The unsafe code must follows the borrow checker rules and is recommended to
// keep the block short with the code that can possibly affect memory safety.

use std::slice;

fn main() {
    // NOTE: Deferencing a raw pointer
    // The raw pointer are similar to references and can be mutable and inmutable,
    // but differ in:
    //  * They can ignore the borrowing rules by having both mutable and inmutable
    // pointers or multiple mutable pointers to the same memory location.
    //  * Don´t guarantee to point to valid memory
    //  * They can be null
    //  * Don't implement automatic cleanup
    //
    // We can create them outside unsafe blocks, but can not deference them outside
    // the block.
    let mut num = 5;
    // Here we create an inmutable raw pointer
    let r1 = &raw const num;
    // Here a mutable raw pointer
    let r2 = &raw mut num;
    // Also, we can create the pointer to an arbitrary memory address.
    // Not recommended.
    let address = 0x012345usize;
    let r = address as *const i32;
    // And to deference them, we need to use the unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // NOTE: Calling unsafe function or method
    // Unsafe functions and methods look like normal ones but with unsafe keyword
    // at the beginning. This way, we indicate the function has requirements that
    // we will met when calling it.
    unsafe fn dangerous() {}
    // we call it inside and unsafe block
    unsafe { dangerous() }

    // NOTE: Safe abstractions over unsafe code
    // It´s not necessary to mark and entire function as unsafe if contains unsafe
    // code. We can wrap the unsafe code in a safe function normally.
    //
    // For example, let's make an implementation of `split_at_mut` function.
    // This method of mutable slices it takes the slice and makes it two by splitting
    // the slice at the index given as an argument.
    //
    // Here the normal use.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // For simplicity, we will implemt it as function instead of method
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len(); // Get the lenght of the slice
        let ptr = values.as_mut_ptr(); // Get the raw pointer of it

        assert!(mid <= len);

        // We may be try this, but by the borrowing rules, it does not work.
        //(&mut values[..mid], &mut values[mid..])
        // In general, borrowing different parts of the slice is ok but rust
        // does not know that, because of this, we must use unsafe code.
        unsafe {
            (
                // This is unsafe code because it takes a raw pointer and must
                // be valid
                slice::from_raw_parts_mut(ptr, mid),
                // Add is algo unsafe because the offset must be a valid pointer
                // too.
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // NOTE: Using `extern` functions to call external code
    // With keyword `extern` Rust code can interact with code written in another
    // language by the *Foreign Function Interface (FFI)*
    //
    // Here, we show how to set up an integration with `abs` function from C
    // standard library. The functions declared as with `extern` blocks are
    // generallly unsafe from Rust code.
    //
    // Here the *"C"* part defines the *Aplication Binary Interface (ABI)* for
    // the external funcion.
    unsafe extern "C" {
        // All functions listed in the block is unsafe. In some cases, if we are
        // sure the function does not have any memory safety consideration, we
        // can use the safe keyboard
        fn abs(input: i32) -> i32;
        //safe fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // NOTE: Calling Rust functions from other languages
    // We can use the `extern` interface to allow other languages to call Rust
    // functions. Instead of the extern block, we add the `extern` kerword and
    // the ABI just before the function definition. Also, we have to annotate to
    // the compiler to not mangle the name of the function by [unsafe(no_mangle)]
    //
    // For example
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!")
    }

    // NOTE: Accessing or modifying a mutable static variable
    // Rust support global variables are similar to *constants* with the sighly
    // difference of global or static variables have a fixed address in memory,
    // which ensure always the access will be to the same data (constants allow
    // duplication); and static variables can be mutable with the unsafe
    // capabilities. We define them with the `static` keyword.
    static mut COUNTER: u32 = 0;

    // It's a good practice when writting unsafe functions to write a comment
    // starting with SAFETY with an explanation of how to call safely the function.
    //
    // SAFETY: Calling this from more than a single thread at a time is undefined
    // behavior, so you *must* guarantee you only call it from a single thread at
    // a time. With multiple threads will resukt in data races.
    unsafe fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
    // In general, for mutable data globally accessible. It's preferable to use
    // the concurrency techniques and thread-safe smart pointers.

    // NOTE: Implementing an unsafe trait
    // A trait is unsafe when at least one of its methods has some variant that
    // the compiler can't verify. They are declared as normal traits with the
    // `unsafe` keyword
    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // NOTE: Using Miri to check unsafe code.
    // Miri is the official Rust tool for detecting undefined behavior, is a
    // dynamic tool that works at runtime. It checks the code by running the
    // program and detecting rules violations.
    //
    // Miri requires a nightly build of Rust. The nightly and stable can be
    // installed by `rustup +nightly component add miri`. And can be run in a
    // project with `cargo +nightly miri run` or `cargo +nightly miri test`
    //
    // NOTE: This is only an introduction to unsafe Rust. For a detailed and
    // deep exploration of how to work effectively with unsafe Rust, read **The
    // Rustonomicon** https://doc.rust-lang.org/nomicon/
}
