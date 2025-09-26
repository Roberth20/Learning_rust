// Match performs good with enums an other patterns
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    // and much more
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    // match is different from 'if' becauses 'if' evaluate boolean expressions,
    // while match evauate any type.
    // And match must cover all the possibilities of the value
    match coin {
        // When the code to execute is long, must be enclose by {} and always 
        // return a value
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // We can bind the match with inner values
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Also, we can use match with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

// Related to all posibilities in match, we can catch all patterns in one
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn game_logic1(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The catch-all pattern must go at the end
        other => move_player(other),
    }
}
fn game_logic2(dice_roll: u8) {
    // if we don't want to use the value when catch-all, we use _
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The catch-all pattern must go at the end
        _ => reroll(), // Can be nothing too, ()
    }
}

fn main() {
    let coins = [
        Coin::Penny, Coin::Nickel, 
        Coin::Quarter(UsState::Alabama), 
        Coin::Quarter(UsState::Alaska)];

    for coin in coins {
        let val = value_in_cents(coin);
        println!("Value of coin is: {val}");
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}