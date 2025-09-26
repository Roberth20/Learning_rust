fn divisors(num: u32) -> Result<Vec<u32>, String> {
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

fn main() {
    let result = divisors(13);
    println!("{result:?}");
    let result = divisors(12);
    println!("{result:?}");
    let result = divisors(25);
    println!("{result:?}");
}