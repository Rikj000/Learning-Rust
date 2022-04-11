fn main() {
    if_else_examples();
    loop_examples();
}

fn if_else_examples() {
    let number = 3;

    //region If-else expression
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    //endregion If-else expression

    //region If not equal expression
    if number != 0 {
        println!("number was something other than zero");
    }
    //endregion If not equal expression

    //region Multiple else-if expressions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //endregion Multiple else-if expressions

    //region Inline if-else expression
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("The value is: {}", value);
    //endregion Inline if-else expression
}

fn loop_examples() {
    //region Labeled + nested loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    //endregion Labeled + nested loop

    //region Return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    //endregion Return value from loop

    //region While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    //endregion While loop

    //region For loop

    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Reverse for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //endregion For loop
}
