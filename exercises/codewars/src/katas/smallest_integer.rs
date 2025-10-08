fn find_smallest(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_smallest(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest(&[34, -345, -1, 100]), -345);
    }
}