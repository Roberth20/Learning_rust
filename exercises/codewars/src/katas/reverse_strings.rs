pub fn spin_words(string: &str) -> String {
    let mut str = String::new();
    for word in string.split_whitespace() {
        if word.len() >= 5 {
            str.push_str(word.chars().rev().collect::<String>().as_str());
        } else {
            str.push_str(word);
        }
        str.push(' ');
    }
    str.trim().to_owned()
}

fn spin_words_v2(string: &str) -> String {
    string.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
    }).collect::<Vec<String>>().join(" ")
}

fn main() {
    let str1 = "Hey fellow warriors";
    let str2 = "This is a test";
    let str3 = "This is another test";

    println!("{}", spin_words(str1));
    println!("{}", spin_words(str2));
    println!("{}", spin_words(str3));
}

#[cfg(test)]
mod tests {
    use super::spin_words;

    #[test]
    fn test() {
        let str1 = "Hey fellow warriors";
        let str2 = "This is a test";
        let str3 = "This is another test";

        assert_eq!(
            spin_words(str1),
            "Hey wollef sroirraw".to_string()
        );
        assert_eq!(
            spin_words(str2),
            "This is a test"
        );
        assert_eq!(
            spin_words(str3),
            "This is rehtona test"
        );
    }
}