/*A restaurant have the front house (the public part) 
and the back house (the private side)

src/main.rs and src/lib.rs are called crate roots because their 
content form the crate root module, with a tree like

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment


Create a module for front house */
mod front_of_house {
    // Inside modules can be other modules, structs, items, enums and more.
    pub mod hosting {
        // Empty functions to work on organization
        /* As childs modules and content are private for parent modules, we must
        sign them as public to access thems */
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn servr_order() {}

        fn take_payment() {} 
    }
}

// To call a function from this module, we need it path. Just like files in the system
pub fn eat_at_restaurant() {
    // Absolute path, from the crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, starts from current module,
    // As the module front_of_house was defined inthis scope, start there.
    front_of_house::hosting::add_to_waitlist();
}

/* When we want to construct a relative path from the parent module
instead from current one, we use super */
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // With super, we call the function from parent module of current (back_of_house)
        super::deliver_order();
    }

    fn cook_order() {}

    /* We can make structs and enums public too, but as the items and fields are inside
    they are private too and we can choose what to left private or set public*/
    pub struct Breakfast {
        pub toast: String, // The use can choose the bread
        seasonal_fruit: String // but can not choose the fruit
    }

    impl Breakfast {
        /* We add the public function to build Breakfast because outside
        the module, the private field is not allowed */
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }
    // In contrast, enums variants are all public
    pub enum Appetizer {
        Soup,
        Salad,
    }

}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change of mind about the bread
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // This line won't compile because we are not allowed to change the
    // seasonal fruit of the meal.
    // meal.seasonal_fruit = String::from("blueberries"):

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// To make easier the use of paths, we can make the name shorter with use
// Here, 'use' bring to scope the hosting module
use crate::front_of_house::hosting;
// The we can call their functions easier
pub fn eat_at_restaurant3 () {
    hosting::add_to_waitlist();
}
// It is important to note, that 'use' bring to the current scope
mod customer {
    // To solved, we must move 'use' to the module
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // Here, the scope is different, so it would fails
        hosting::add_to_waitlist();
    }
}