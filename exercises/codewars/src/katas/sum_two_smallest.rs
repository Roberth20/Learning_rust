pub fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut mut_numbers = numbers.to_owned();
    mut_numbers.sort();
    mut_numbers[0] + mut_numbers[1]
}

/*use itertools::Itertools;

fn sum_two_smallest_numbers_v2(numbers: &[u32]) -> u32 {
    numbers.iter().k_smallest(2).sum()
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = [19, 5, 42, 2, 77];
        assert_eq!(sum_two_smallest_numbers(&nums), 7);
    }
}