// Define a debuggable enum with all the united states
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

// Define a coin enum
enum Coin { Penny, Nickel, Dime, Quarter(UsState) }

fn main() {
    // Print the values of the coins in cents
    println!("Penny value in cents is: {}", value_in_cents(Coin::Penny));
    println!("Nickel value in cents is: {}", value_in_cents(Coin::Nickel));
    println!("Dime value in cents is: {}", value_in_cents(Coin::Dime));
    println!("Quarter value in cents is: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Quarter value in cents is: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // Print the values of the plus one results
    let five = Some(5);
    println!("five: {:?}", five);
    println!("six: {:?}", plus_one(five));
    println!("none: {:?}", plus_one(None));

    // Handle some hard coded dice roll results
    dice_roll_result(3);
    dice_roll_result(7);
    dice_roll_result(9);
}

// Fetch the value in cents through a match pattern
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// Attempt to add one to X and return it through a match pattern
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

// Handle the result of the dice roll through a catch all other match pattern
fn dice_roll_result(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // other wildcard to use match value
        _ => re_roll(), // _ wildcard to ignore the match value
    }
}

fn add_fancy_hat() { println!("Player received fancy hat!") }
fn remove_fancy_hat() { println!("Player lost fancy hat...") }
fn move_player(dice_roll: u8) { println!("Player moved {} spaces...", dice_roll)}
fn re_roll() { println!("Re-rolling dice...") }