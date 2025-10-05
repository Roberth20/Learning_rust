pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result = Vec::new();
    for (age, handi) in data {
        if (age < 55) | (handi < 8) {
            result.push("Open".to_string());
        } else {
            result.push("Senior".to_string());
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
    }
}