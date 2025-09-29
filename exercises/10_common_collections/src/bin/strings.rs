fn main() {
    // We can create strings in mutiple ways
    let mut s0 = String::new();
    let s1 = "world".to_string();
    let s2 = String::from("Hello");
    // The strings are UTF-8 encoded, allowing to write all types of characters
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // We can updtae strings in different ways too.
    // This does not take ownership of the 'hello'
    s0.push_str("hello");
    s0.push(',');
    // by concatenation, this operator take ownership of first value and the
    // reference of the second to return a new one.
    let mut s3 = s0 + &s1;
    // For more complicated strings joins, we can use format!
    let s = format!("{s3}!, {hello} mundo!"); // Don't take ownership
    println!("{s}");

    // Rust does not allow indexing Strings, because the nature of bytes that their 
    // lenght might change. Because of this, we have to use slices but they can fail
    let hello = "Здравствуйте";
    // This works because capture the 4 bytes that represent the first 2 characters
    let s = &hello[0..4];
    // This fails because capture half of bytes that make the character
    //let s = &hello[0..1];

    // To iterate over strings, it is better to use the methods that allow to choose
    // between bytes or chars
    for c in "Зд".chars() {
        println!("{c}");
    }
    for c in "Зд".bytes() {
        println!("{c}");
    }

}