use crate::UsState::{Alabama, Alaska};

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
    let config_max = Some(3u8); // Max u8 value

    // Handle only 1 result, ignore the rest with a match + wildcard
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => () // Ignore all other options
    }

    // Same result as above through if-let control flow, less boiler-plate, but less modifiable.
    if let Some(max) = config_max {println!("The maximum is configured to be {}", max); }

    // Initialize some coins + a quarter counter
    let coin1 = Coin::Penny;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Nickel;
    let coin4 = Coin::Quarter(Alaska);
    let coin5 = Coin::Quarter(Alabama);
    let mut count: usize = 0;

    // Count through match case + wildcard
    count = check_coin_state_match(&coin1, count);
    count = check_coin_state_match(&coin2, count);

    // Count through if let
    count = check_coin_state_if_let(&coin3, count);
    count = check_coin_state_if_let(&coin4, count);
    count = check_coin_state_if_let(&coin5, count);

    println!("Total quarters counted: {}", count);
}

// Concise Control Flow with match case + wildcard
fn check_coin_state_match(coin: &Coin, mut count: usize) -> usize {
    match coin {
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            count += 1;
        }, _ => ()
    }
    count
}

// Concise Control Flow with if let
fn check_coin_state_if_let(coin: &Coin, mut count: usize) -> usize {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count += 1;
    }
    count
}