// Let's try to build a function that returns the first word in a string
fn main() {
    let s = String::from("Hello, world!");

    println!("{} world!", first_word(&s));
}

// This way, the first word will be
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Rust counts with the slices type
            // with .. been the range syntax
            return &s[0..i]; // Equivalent to &s[..i]
        }
    }

    &s[..]
}