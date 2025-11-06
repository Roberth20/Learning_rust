/* If we try to make GUI which iterate over items calling draw function to make them
show in the screen. We may try to use a vector with enums, but this only work if the 
variants are known before hand and are limited. 

To extend it, we use Trait Objects.*/

// Start creating the Trait Draw for common behavior
trait Draw {
    fn draw(&self);
}
/* Then we implement the trait in a vector as a Trait object. They are created
with any pointer like & or Box<T>, following dyn keyword and the trait. They allow to 
point both the instance of the type that implement the trait and the table with the 
methods included with the trait*/
struct Screen {
    // A list of Boxes of any type which implement Draw trait
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Now, implementing
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // The code to draw a box in GUI
        todo!()
    }
}

fn main() {

}