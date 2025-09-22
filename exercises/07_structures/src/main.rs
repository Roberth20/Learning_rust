// Let's explore the structs againts other types to calculate the area of a rectangle

// Also, it useful when developing print the structure to debug, for that we explicitly
// add the functionality
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // With simple variables
    let height1 = 30;
    let width1 = 50;
    
    println!(
        "The area of the square is {} square pixels",
        area1(height1, width1)
    );

    // With tuples
    let rect1 = (30, 50);

    println!(
        "The area of the square is {} square pixels",
        area2(rect1)
    );

    // With stucts
    let rect2 = Rectangle {
        height: 30,
        width: 50
    };

    // Print the debug dtructure
    println!("rect2 is {rect1:#?}");
    // Another option to debug is using the macro dbg!
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect3); // dbg! takes ownership and then returned. Because that, send the reference


    println!(
        "The area of the square is {} square pixels",
        area3(&rect2)
    );
}

fn area1(height: u32, width: u32) -> u32 {
    // This way works, but we lost the relation with this approach
    // we may lost the meaning and make it harder to debug
    height * width
}

fn area2(rect: (u32, u32)) -> u32 {
    // This approach keep better the data information but it not enough
    // the information related to 0 and 1 methods.
    rect.0 * rect.1
}

fn area3(rect: &Rectangle) -> u32 {
    // This approach keep better the data information but it not enough
    // the information related to 0 and 1 methods.
    rect.width * rect.height
}