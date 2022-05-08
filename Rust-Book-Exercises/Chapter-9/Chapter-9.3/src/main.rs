use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::net::IpAddr;

pub struct Guess { value: u32 }
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 { self.value }
}

fn main() {
    // We know this hard coded value will parse, but still need to handle errors with unwrap
    let _home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("Guess the number v2!");

    // Generate a random secret number between 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number); // Debug line, prints answer

    // Loop until answer found
    loop {
        // Ask the user for input and parse it as an integer, handle errors
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() { Ok(num) => num, Err(_) => continue };
        let guess_struct: Guess = Guess::new(guess);
        println!("You guessed: {}", guess_struct.value());

        // Compare the random number with the user input, end loop if correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}