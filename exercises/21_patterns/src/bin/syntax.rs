// Some common syntax patterns that might want to use.

fn main() {
    // NOTE: MATCHING LITERALS
    // We can match the patterns agains literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // NOTE: MATCHING NAMED VARIABLES
    // named variables are irrefutable patterns that match any value but in expressions
    // that open a new scope they can be shadow the old ones.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    // In this example, the match create a variable `y` that shadows the old one in the scope
    // When the scope end, the variable come back to previous value.
    println!("at the end: x = {x:?}, y = {y}");

    // NOTE: MULTIPLE PATTERNS
    // In `match` expressions we can use the operator `|` to match multiple patterns.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // NOTE: MATCHING RANGES OF VALUES
    // With `..=` syntax we can match with an inclusive range of values.
    let x = 5;
    // match with numbers
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // The compiler checks that the range isn't empty, because that the only types
    // allowed for this syntax are numeric and `char`
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // NOTE: DESTRUCTURING
    // Thus patterns are useful to destructure strucs, enums and tuples
    //
    // Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    // This is the normal way
    //let Point { x: a, y: b } = p;
    // When using the same variable name as the fields, we can use the shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also detructure in the pattern without separate the values first
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axus at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    // Enums
    // In general, destructuring enums depends on the way the data is stored
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    //  NOTE: DESTRUCTURING NESTED STRUCTS AND ENUMS
    //  The matching patterns works with structures too.
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Messages {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Messages::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Messages::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Messages::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
    // We can go further in more complicated ways
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // NOTE: IGNORING VALUES IN PATTERN
    // There are mainly three ways to ignore a pattern. `_` match anything to ignore, like
    // in `match`arms. With `_`+pattern, like with a name, and finally `..` to ignore
    // the remaining parts of a value.
    //
    // A complete value with `_`
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    foo(3, 4);

    // Parts of value with a nested `_`
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting in {setting_value:?}");
    // Also we can ignore some values from a pattern
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // Unused variables with name starting with `_`
    // This is useful when prototyping or starting a project.
    let _x = 5;
    let y = 10;
    // But there is a difference with only `_`, here the variable is binded to the value
    // and with `_` is ignored completely
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
}
