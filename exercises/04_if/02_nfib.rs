fn main() {
    let n: u32 = 8;
    let mut current_number: u32 = 1;
    let mut previous_number: u32 = 1;

    for iter in 1..(n+1) {
        println!("{iter}th number: {current_number}");
        let holder = current_number;
        current_number += previous_number;
        previous_number = holder;
    }
}