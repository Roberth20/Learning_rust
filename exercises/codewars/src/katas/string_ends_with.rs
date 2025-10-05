pub fn solution(word: &str, ending: &str) -> bool {
    // short way, word.ends_with(ending)
    if ending.len() <= word.len() {
        &word[word.len() - ending.len()..] == ending
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert!(solution("abc", "c"));
        assert!(!solution("strawberry", "banana"));
    }
}