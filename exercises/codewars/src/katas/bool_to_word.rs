fn bool_to_word(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::bool_to_word;

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
}
