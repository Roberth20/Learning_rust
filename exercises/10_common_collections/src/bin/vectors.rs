fn main() {
    // We can create vectors by using Vec::new() or with the macro vec!.
    // They can hold only one type of data
    let vector: Vec<i32> = Vec::new();
    let mut vector: Vec<i32> = vec![1, 2, 3];

    // We can update the vecto with different methods, like push
    vector.push(4);

    // We can access to values in vector by two ways
    let third: &i32 = &vector[2];
    print!("The third element is {third}");

    let third: Option<&i32> = vector.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is not a third value.")
    }
    
    // To iterate, we must borrow or take as mutable
    for i in &vector {
        println!("{i}");
    }
    for i in &mut vector {
        // To edit the value of the reference, we have to deference (*)
        *i *= 2;
        println!("{i}");
    }

    // Using enums we can make a vector store multiple value types inside one type
    enum SpreedSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreedSheetCell::Int(3),
        SpreedSheetCell::Float(3.1416),
        SpreedSheetCell::Text("This is Pi".to_string())
    ];
    
}
