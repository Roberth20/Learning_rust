fn main() {
    // Adding mut allow a variable to be mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Constant are fixed values, always inmutable and
    // with data type annotated
    const ONE_DAY_IN_SECONDS: u32 = 60*60*24;
    println!("A day have {ONE_DAY_IN_SECONDS} seconds");
    // We can shadow a variabe to create a new one with the
    // same name an different value
    let x = 5;
    let x = x + 1;
    // This is a scope, like an isolated code section.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The value of x is: {x}");
    
    // Shadowing is different from mutable to make transformations
    // and data type changes. mutable variables does not change the type.
    let spaces = "    ";
    print!("Before shadowing: {spaces}");
    let spaces = spaces.len();
    print!("After shadowing: {spaces}");
}
