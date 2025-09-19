use std::io; // Import IO module
use std::cmp::Ordering; // Import type Ordering
// Implement a trait of random numbers, seen in Chapter 10.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //genrate a random number with the thread of OS
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    // Adding a loop
    loop {
        println!("Please input your guess.");

        // 'let' allow to create variables
        // 'mut' convert a variable as mutable one.
        // String::new() create a new empty string
        let mut guess = String::new(); 

        // stdin() receive input from user.
        // after that, the input readed is stored in guess
        // & is for references, &mut rferences the variable in a mutable way
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // expect allow to handle errors, always there must be a way to handle them.

        // To convert, 'trim()' to eliminate white spaces at eginning and end.
        // then 'paerse()' convert to the indicated type.
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Instead to crash with a non-number input, let's add handling input with match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Variables are printed with {}, for expresion, must be empty, like
        // println!("You guessed {}", guess + 1);
        println!("You guessed: {guess}");

        // Guessing logic
        // cmp method compare two values and return a variant of Ordering,
        // then we use 'match' to decide what to do with the returned values.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            // Now, adding exit when correct guess
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}