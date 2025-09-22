// We can wrap the area calculation to the structure converting it to a method
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
// The methods exists in the context of the struct and their first parameter 
// is always self.
impl Rectangle { // impl -> implementation
    fn area(&self) -> u32 { // &self is an abbreviation for self: &self where self
        // is the instance. Also we borrow the instance to not take ownership.
        self.height * self.width
    }
}
// In general, methods reference the instance in a mutable o inmutable way.
// They don't take ownership unless transform the instance

// Also, we can name a method like a fiel
impl Rectangle {
    // This kind of methods are useful to return the value of the field
    fn width(&self) -> bool {
        self.width > 0
    }
}
// Now, we practice adding a method to determine if a rectangle fit inside other
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        (self.width > rect.width) & (self.height > rect.height)
    }
}
// Aditionally, we can add related functions to the struct which don't have self.
// Thus functions usually are used to create special instances of the struct
impl Rectangle {
    // This function create a special instance square from Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50
    };
    let rect2 = Rectangle {
        height: 20,
        width: 30
    };
    // To use implied functions we use ::
    let sq = Rectangle::square(30);
    
    if rect1.width() {
        // Here, rust recognize the field when not called with ()
        println!(
            "The rectangle has a nonzero width: {}",
            rect1.width
    );
    }

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}
