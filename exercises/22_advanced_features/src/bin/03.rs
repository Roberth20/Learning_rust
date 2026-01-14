// TODO: Fix the compiler error without taking the macro definition out of this
// module.
//

// There are many ways to resolve this. I will choose just use macro_export
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
