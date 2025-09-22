struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64
}
// Also we can create structs like tuples, without field names
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// We can start the instance as an expression
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // We can start the value with the shorthand if the name are the sames
        email, // email: email -> email
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("robmago"),
        email: String::from("hello@world.com"),
        sign_in_count: 1
    };
    // We can access and edit (if mutable) the values as dot methods
    user1.email = String::from("someonemail@mail.com");

    // Also, we can create an instance from another instance, keeping some values
    let user2 = User {
        email: String::from("Hello@world.com"),
        ..user1 // ..user1 define that the rest of fields are the same
    }; // To remember, in this case we are copying a String from user1 to user2, so 
       // user1.username is no longer active by ownership. 

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
