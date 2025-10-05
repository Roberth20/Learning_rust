pub fn divisors(num: u32) -> Result<Vec<u32>, String> {
    /* Calculate the divisors of a number and return them
    as an array. If the number is prime, then retun a message.
    */
    let mut arr = Vec::new();
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            arr.push(i);
        }
    }
    if arr.is_empty() {
        Err(format!("{num} is prime!"))
    } else {
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(divisors(13).unwrap_err(), "13 is prime!");
        assert_eq!(divisors(12).unwrap(), vec![2, 3, 4, 6]);
        assert_eq!(divisors(25).unwrap(), vec![5]);
    }    
}