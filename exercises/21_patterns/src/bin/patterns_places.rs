fn main() {
    // Can be used in match arms, like
    //  ```
    //  match VALUE {
    //      PATTERN => EXPRESSION,
    //      PATTERN => EXPRESSION,
    //  }
    //  ```
    //  Where the match expressions must be exhaustive, where the optional '_'
    //  will match anything.
    let x = Some(1); // Can be some value or none 
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    //  Also in let statements like
    //  ```
    //  let PATTERN = EXPRESSION;
    //  ```
    //  This can be as complex as we want, as always the pattern match the expression
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    //  This does not work because failing matching
    //  let (x, y) = (1, 2, 3);
    
    //  In conditional 'if let' expressions for control flow like match for only
    //  one case.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // Here we shadow the age previous value, for that reason we can not combine
    // the two conditions in one.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    // Also, this pattern is not exhaustive so it could lead logic bugs.
    } else {
        println!("Using blue as the background color");
    }

    //  'while let' conditional loops
    //  This is useful to continue looping while the pattern match
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    //  for loops
    //  The value that follows the keyboard 'for' is always a pettern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    //  Function parameters
    //  They can be simple like normal declations
    fn foo(x: i32) {
        println!("The number is {x}");
    }
    //  Or like let, to unpack values
    fn coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    coordinates(&point);
}
