/* Generics allow to remove duplicated code allowing to do operations in
different data types*/

/*FOR FUNCTIONS

Taking for example two functions that return the largest value of an array,
one for integers another for chars*/
fn largest_i32(list: &[i32]) -> &i32 {
    let mut number = &list[0];

    for item in list {
        if item > number {
            number = item;
        }
    }

    number
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Those functions do the same, so we make a new one with generic type, which needs a 
// identifier and by conventionts, for generic type is T. Also, we must declare the parameters
// with the function name

// Note: To compile, we need to declare the trait PartialOrd, to restrict T to types
// with order
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* IN STRUCT 
To define generics in struct, works in a similar way to functions*/
struct Point<T> {
    // Here as we only defined one generic for both x,y, their types must be
    // the same.
    x: T,
    y: T
}
// A struct with mixed types
struct Point2<X1, Y1> {
    x: X1,
    y: Y1
}

/* FOR ENUMS 

We saw some examples with Option<T> and Result<T, E>*/

/*FOR METHODS 

* When defined the generics with impl, we can use the generic with all the methods.
* Also, we can define methods for a given type as constraint
* Finally, the generics can be diferent from the struct
*/
impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}
impl Point<f32> {
    // This methods only exists if the type of the generic is f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// Here the impl block have the generics associeated to the struct
impl<X1, Y1> Point2<X1, Y1> {
    // And for this particular method, we use other generics
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 { 
            x: self.x, 
            y: other.y 
        }
    }
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");
    let result = largest(&number_list);
    println!("The largest by generics number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
    let result = largest(&char_list);
    println!("The largest by generics char is {result}");

    let integer = Point{x: 5, y: 10};
    let float = Point{x: 5.0, y: 10.0};
    let integer = Point2{x: 5, y: 10};
    let float = Point2{x: 5.0, y: 10.0};
    let mixed = Point2{x: 5, y: 10.0};

    let p = Point{x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p: Point<f32> = Point{x: 5.0, y: 10.0};
    println!("Distance from origin for type f32: {}", p.distance_from_origin());

    let p1 = Point2{x: 5, y: 10.0};
    let p2 = Point2{x: "Hola", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}