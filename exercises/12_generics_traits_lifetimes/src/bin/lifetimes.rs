/* Trying to write a function to return the longest string.

This function won't compile because the borrow checker can not determine the lifetime
or scope of the parameters

fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

The way we write the lifetimes is with a ' following a generic name, normaly, a.
* &i32 normal reference
* &'a i32 a reference with explicit lifetime
* &'a mut i32 a mutable reference with explicit lifetime
*/
// The lifetime generics tells the function that the result lifetime will be valid
// only if the parameters lifetime is valid
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Lifetimes can be used in structs to hold references
struct ImportantExcept<'a> {
    part: &'a str
}

fn main() {
    let string1 = String::from("abcde");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest strings is: {result}");
    
    // Here string1 will be valid for all the scope
    let string1 = String::from("long string is long");

    {
        // But string2 and result will be valid only in the inner scope
        let string2 = String::from("xyz");
        // The lifetime of longest result, will be the lowest of the parameter
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    /* If we want to use result in the outer scope, when string2 is out of scope, 
     it will fail
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
    */

    let novel = String::from("Call me Ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().unwrap();
    // This way, ImportantExcept hold a reference to first sentence and live while novel is on the scope
    let i = ImportantExcept{
        part: first_sentence,
    };

}

// The lifetime must be related to the parameters of the function
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    // This function only return x, so it doesn't need to validate lifetime in y
    x
}

/* Also, when returning a reference from a function, the lifetime must be the same
of one of the parameters it takes. because of this, the function below won't compile
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("A very long string.");
    result.as_str()
}*/