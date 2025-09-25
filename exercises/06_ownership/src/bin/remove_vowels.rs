// String cleaner to remove vowels
fn remove_vowels_with_ownership(string: String) -> String {
    let mut clean_str = String::new();
    for character in string.to_lowercase().chars() {
        if !"aeiou".contains(character) {
            clean_str.push(character);
        }
    }
    String::from(clean_str)
}

fn remove_vowels_with_borrowing(string: &String) -> String {
    let mut new_string = String::new();
    
    for character in string.to_lowercase().chars() {
        if !"aeiou".contains(character) {
            new_string.push(character);
        }
    }
    
    new_string
}

fn main() {
    let text = String::from("Hola mundo!");
    println!("Removing vowels taking ownership");
    println!("{text}");
    let clean_text = remove_vowels_with_ownership(text);
    println!("{clean_text}");

    let text = String::from("Hola mundo!");
    println!("Removing vowels borrowing");
    let clean_text = remove_vowels_with_borrowing(&text);
    println!("{text} -> {clean_text}");
}