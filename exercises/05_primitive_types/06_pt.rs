fn main() {
    let numbers = (1, 2, 3);

    // TODO: Use a tuple index to access the second element of `numbers`
    // and assign it to a variable called `second`.
    // let second = ???;
    let second = numbers.1;
    if second != 2 {
        println!("This is not the 2nd number in the tuple!");
    }
    println!("Very good!");
}