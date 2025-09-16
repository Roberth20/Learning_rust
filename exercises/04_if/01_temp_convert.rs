fn main() {
    let celsius: f32 = 20.0;
    let fahr: f32 = 140.0;

    println!("{celsius} celsius are {} fahrenheit", cel_to_fahr(celsius));
    println!("{fahr} fahrenheit are {} celsius", fahr_to_cel(fahr));
}

fn cel_to_fahr(cel: f32) -> f32 {
    cel * (9.0 / 5.0) + 32.0
}

fn fahr_to_cel(fahr: f32) -> f32 {
    (fahr - 32.0) * (5.0 / 9.0)
}