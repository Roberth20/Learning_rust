pub fn add_two(a: u64) -> u64 {
    a + 2
}
// Now, it's important to test that all structs and enums we implement must have
// the attributes Debug and PartialEq to be run properly by the assertions
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Imagine a function that greets people by name
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// Taking from the gussing game, it depends on the value of the instance is between 1 and 100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

/* We can control the test behavior from cargo in two ways:
 * for cargo test and compiler, check cargo test --help
 * for the test binary, are separated by --, check them with cargo test -- --help

Some common uses for binaries are:
* Change the test parallel running, by default run in parallel, we can change it like 
    cargo test -- --test-threads=n
* Show output from functions, like println!.
    cargo test -- --show-output
* To run one or more test, we give a test name to match all the text name that have
such value in them
    cargo test <test-name>
* To run only the ignored test,
    cargo run -- --ignored
*/
// Unit tests goes here, inside each module file. And #[cfg(test)] tels to Rust this code
// will only compile and run with cargo test
#[cfg(test)] // cfg -> configuration
mod tests {
    // The tests are inside a module, for this reason we need to bring to scope the
    // code from outer module
    use super::*;

    // WE identify test with the attributte test, this make a function to be run as test
    #[test]
    fn test_add() {
        let result = add_two(2);
        // with assert_eq and assert_ne we can check if the result is the same or 
        // different to a expected value
        assert_eq!(result, 4);
    }
    // A test fail when it panics, for example
    #[test]
    #[ignore] // To mark ignore the test and not execute it
    fn test_fail() {
        panic!("Failed test!");
    }

    // With assert! we check boolean values, if result is false it panics
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        // Because the result is false, we need to negate the result to pass the assert
        assert!(!smaller.can_hold(&larger));
    }

    // The test fot the greeting, as the greeting may change, needs to check the name
    #[test]
    fn greeting_contains_name() {
        // We can improve the assertion message if fails by give a message to the assert
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        )
    }

    // To test when the code should panic, we use should_panic attribute
    #[test]
    // With should panic it can pass the test with whatever the reason for panic
    // To make it more accurate, we give it a expected text to match the message
    #[should_panic(expected = "value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    // Finally, we can use Result<T, E> in test
    #[test]
    // With this annotation, we can use should_panic
    fn it_works() -> Result<(), String> {
        let result = add_two(2);
        // This way return a error messahe instead panicking
        if result == 4 {
            // Return Ok() when the test pass
            Ok(())
        } else {
            // And Err with string if fails
            Err(String::from("Two plus two does not equal four"))
        }
    }
}