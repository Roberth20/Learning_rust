fn perimeter(n: u64) -> u64 {
    // As the rectangles follow the fib seq, we just multiply by 4 the sum of perimeters
    // of n-1 rectangles of fib seq size
    let mut current_number: u64 = 1;
    let mut previous_number: u64 = 1;
    let mut sum: u64 = 1;

    for i in 1..(n+1) {
        sum += current_number;
        let holder = current_number;
        current_number += previous_number;
        previous_number = holder;

    }
    sum * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }

    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}