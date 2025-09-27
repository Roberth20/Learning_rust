
// With use, we bring to scope the path we refer to
use crate::garden::vegetables::Asparagus;

// Declare the module to tell the compiler to use the code from this module
// BY default, modules are private
pub mod garden;

fn main() {
    println!("This is the root binary crate!");
    println!("The main folder, where is the Cargo.toml, is the package.");
    // Use the path we bring to scope
    let plant = Asparagus{};
    println!("I'm growing {plant:?}!");
}
