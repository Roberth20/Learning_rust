pub fn solution(s: &str) -> String {
    s.chars().map(
        |c| if c.is_ascii_uppercase() {
            format!(" {c}")
        } else {
            c.to_string()
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
        assert_eq!(solution(""), "");
    }
}