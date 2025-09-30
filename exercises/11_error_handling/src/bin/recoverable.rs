/*As the enum Result have two type of values (Ok, Err), we can use them to handle errors */

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    // This return a Result type
    let greeting_file_result = File::open("hello.txt");
    let experiment = 3;

    // A simple way to handle Result is with match
    if experiment == 0 {
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file {error:?}")
        };
    }
    let greeting_file_result = File::open("hello.txt");
    if experiment == 1 {
        // Also, we can take actions depending of the error
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            // match the error by the kind
            Err(error) => match error.kind() {
                // Crea el archivo si no fue encontrado
                ErrorKind::NotFound => match File::create("hello.txt") {
                    // Return created file
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {e:?}")
                },
                _ => panic!("Problem opening the file {error:?}")
            }
        };
    }

    // As shortcut for match, we can use unwrap()
    if experiment == 2 {
        // This will return the value from Ok and call panic! if Err
        let greeting_file = File::open("hello.txt").unwrap();
    }
    // Also we can use expect(), which do the same as unwrap() but it let us choose the
    // panic message
    if experiment == 3 {
        let greeting_file = File::open("hello.txt")
        .expect("hello.txt must be included in the project");
    }
}

// Instead to handle the error inside the function, we can propagete the error and
// allow other function to handle it

// We choose io::Error for the signature because is the error returned from the operators
fn read_username_from_file1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// The propagation is so common that we have the operator ?
fn read_username_from_file2() -> Result<String, io::Error> {
    // ? do the same as prevoues match, return the value if Ok and
    // return as result of whole function the Err
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// With ? we can compact a lot more the function
fn read_username_from_file3() -> Result<String, io::Error> {
    // This operator only work if the signature of type we return is the same as defined
    // in the function
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// To finish, this operation is so common that we can use
fn read_username_from_file4() -> Result<String, io::Error> {
    use std::fs;
    fs::read_to_string("hello.txt")
}

// The operator ? also works with Option<T> and Null values
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/*Finally, main can return different types, listed in 
https://doc.rust-lang.org/std/process/trait.Termination.html */