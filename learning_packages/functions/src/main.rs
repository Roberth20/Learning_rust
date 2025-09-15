fn main() {
    println!("Hello, world!");

    another_function();

    parameter_function(5);

    parameters_function(5, 'h');

    // Scope blocks are expressions which return values
    let y: u32 = {
        let x: u32 = 3;
        // Expressions don't have ;, if have ; it converts to a statement
        x + 1
    };
    println!("Value of y = {y}");

    let y = return_function(y);

    println!("Multiplied y = {y}");
}

fn another_function() {
    println!("Another function.");
}

fn parameter_function(x: u32) {
    println!("The parameter x has value: {x}");
}

fn parameters_function(value: u32, unit_label: char) {
    println!("The measurement is {value} {unit_label}");
}

// To return values, me must -> specify the value type
fn return_function(x: u32) -> u32 {
    x * 2
}