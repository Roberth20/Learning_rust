fn capitalize(mut array: Vec<String>) -> Vec<String> {
    // Iterate over the array borrowing the values
    for item in array.iter_mut() {
        // Check is not empty string
        if !item.is_empty() {
            item[..1].make_ascii_uppercase();
            item[1..].make_ascii_lowercase();
        }
    }
    array
}

fn main() {
    let mut arr = Vec::new();
    arr.push(String::from("hola"));
    arr.push(String::from("MUNDO"));
    arr.push(String::from(""));
    let new_array = capitalize(arr);
    for item in new_array {
        println!("{item}");
    }
}