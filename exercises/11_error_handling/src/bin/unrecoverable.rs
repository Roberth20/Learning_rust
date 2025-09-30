/* Unrecoverable errors are ones that make the program fails by things
beyond the programmer did. Like access to an index in array that not exists.
For this, we call the macro panic! */
fn main() {
    /* panic! stop the execution and starts to clearing the memory, this is slow
    anf if we want to let the OS clean the memory instead, i.e. smaller files.
    We have to set in Cargo.toml: 
    [profile.release]
    panic = 'abort' */
    panic!("Crash and run!");
}