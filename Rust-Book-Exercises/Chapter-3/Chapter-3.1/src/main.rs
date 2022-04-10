fn main() {
    // Define & print a immutable variable
    let x = 10;
    println!("The value of immutable variable x is: {}", x);

    // Define & print a mutable variable
    let mut y = 5;
    println!("The value of mutable variable y is: {}", y);
    y = 6;
    println!("The value of mutable variable y is: {}", y);

    // Define & print a constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // Define a immutable variable and shadow it
    let z = 5;
    println!("The value of immutable variable z is: {}", z);
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of shadowed immutable variable z in the inner scope is: {}", z);
    }
    println!("The value of shadowed immutable variable z is: {}", z);

    // Define a immutable variable and shadow it's type
    let dots = "...";
    println!("The value of immutable variable dots is: {}", dots);
    let dots = dots.len();
    println!("The value of shadowed immutable variable dots is: {}", dots);
}
