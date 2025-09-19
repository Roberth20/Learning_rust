fn main() {
    let number: u32 = 6;
    
    // Simple condition, always must be bool
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
    // Only if condition
    if number != 0 {
        println!("The number is different from zero");
    }
    // Multiple if statements
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
    // In general, more than one if is not necessary, maybe the code can be refacted
    // or may require the use of 'match'

    // As if is a expression, we can use it with let
    let condition: bool = true;
    let number: u32 = if condition {5} else {6};

    println!("The value of number is {number}");

    // The loop is the first repeating option
    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    // When multiple loops, 'continue' and 'break' apply to inner loop. To help to
    // control, we can label the loop with "'"
    let mut count: u32 = 0;
    // loop label
    'counting_up: loop {
        println!("Count {count}");
        let mut remaining: u32 = 10;

        loop {
            println!("Remaining {remaining}");
            if remaining == 9 {
                // break inner loop
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    
    // The when the condition must be evaluated in each loop, instead inside loop flow
    // we use while
    let mut number: u32 = 3;

    while number != 0 {
        println!("number: {number}");

        number -= 1;
    }
    println!("Finished regressive count.");
    // The while can be used to navigate the elements in a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // This way is error prone and not efficient. Because of this, we have the for
    // loop
    let a: [u32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    // To perform a for loop a number of times, we use range.
    for number in 1..5 {
        println!("range counter {number}!");
    }
    // In this case, we use reversed range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LAUNCH!");
}
