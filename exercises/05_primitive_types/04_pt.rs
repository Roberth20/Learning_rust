fn main() {
    let a = [0, 1, 2, 3, 4, 5];

    // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
    // let nice_slice = ???
    let nice_slice = &a[2..5];

    println!("[2, 3, 4] == nice_slice: {}", nice_slice == [2, 3, 4]);
}