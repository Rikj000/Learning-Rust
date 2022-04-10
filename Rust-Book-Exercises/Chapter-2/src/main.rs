use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

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
        println!("You guessed: {}", guess);

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
