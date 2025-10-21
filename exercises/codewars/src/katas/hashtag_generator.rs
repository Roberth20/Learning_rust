pub fn generate_hashtag(s: &str) -> Option<String> {
    if s.is_empty() {
        return None;
    }
    let result = format!("#{}",
        s.split_whitespace()
        .map(|w|
            format!("{}{}",
                &w.trim()[..1].to_ascii_uppercase(),
                &w.trim()[1..].to_ascii_lowercase(),
            )
        )
        .collect::<String>()
    );
    if result.len() > 140 {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::generate_hashtag;

    fn dotest(s: &str, expected: Option<String>) {
        let actual = generate_hashtag(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) when testing with s = {s:?}"
        );
    }

    #[test]
    fn fixed_tests() {
        dotest("", None);
        dotest("Codewars", Some("#Codewars".to_owned()));
        dotest("Codewars      ", Some("#Codewars".to_owned()));
        dotest("Codewars Is Nice", Some("#CodewarsIsNice".to_owned()));
        dotest("codewars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("CodeWars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("c i n", Some("#CIN".to_owned()));
        dotest("codewars  is  nice", Some("#CodewarsIsNice".to_owned()));
        dotest("Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong Cat",
            None);
    }
}
