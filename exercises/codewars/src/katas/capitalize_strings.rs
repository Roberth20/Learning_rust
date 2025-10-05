pub fn capitalize(mut array: Vec<String>) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut arr = Vec::new();
        arr.push("hola".to_string());
        arr.push("MUNDO".to_string());
        arr.push(".".to_string());
        assert_eq!(
            vec!["Hola", "Mundo", "."],
            capitalize(arr)
        );
    }
}