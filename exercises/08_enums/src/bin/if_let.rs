#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    // and much more
}
// The most common pattern is perform a task with a value and return a default otherwise
impl UsState {
    fn existed_id(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // bla bla bla
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}


// Then we use the if let pattern
fn describe_state_quarter1(coin: Coin) -> Option<String> {
    // This code works but can be difficult to follow with complex data
    if let Coin::Quarter(state) = coin {
        if state.existed_id(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
fn describe_state_quarter2(coin: Coin) -> Option<String> {
    // As match and if are aexpressions and return values, we could use them
    // to return the value and make it clearer
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_id(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter3(coin: Coin) -> Option<String> {
    // The two previous functions do the job but we can make the cleanest
    // possible by using let .. else syntax. This allow us to compare an expression
    // and return it if the conditions match, like if let but without the if.
    let Coin::Quarter(state) = coin else {
        return None;
    };
    
    if state.existed_id(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    // For example this match pattern
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => println!("You must set a maximum!")
    }
    // This pattern, when we perform an action only if there is a value is annoying.
    // In such case, we use if let equivalent
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // We can add an else to replicate from match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("You must set a maximum")
    }
}