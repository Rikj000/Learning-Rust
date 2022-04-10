use std::io;

fn main() {
    // Parse a string to an unsigned 32 bit integer
    let _number: u32 = "42".parse().expect("Not a number!");

    //region Floating Points
    let _a = 2.0; // f64 - Default as 64 bit
    let _b: f32 = 3.7; // f32 - Explicitly as 32 bit
    //endregion Floating Points

    //region Basic numerical math operations
    let _sum = 5 + 10;            // Addition
    let _difference = 95.5 - 4.3; // Subtraction
    let _product = 4 * 30;        // Multiplication
    let _quotient = 56.7 / 32.2;  // Division
    let _floored = 2 / 3;         // Results in 0
    let _remainder = 43 % 5;      // Remainder
    //endregion Basic numerical math operations

    //region Booleans
    let _t = true;  // Default type annotation
    let _f: bool = false; // Explicit type annotation
    //endregion Booleans

    //region Characters
    let _c1 = 'z';
    let _c2 = 'ℤ';
    let _heart_eyed_cat = '😻';
    //endregion Characters

    //region Tuples

    // Initialize tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Directly access tuple values
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // Destructure and access tuple values
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    //endregion Tuples

    //region Arrays

    // Initialize array types
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"]; // Default type annotation
    let arr1: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type annotation
    let _arr2 = [3; 5]; // Same value annotation = [3, 3, 3, 3, 3]

    // Access array values
    let _first = arr1[0];
    let _second = arr1[1];

    // Test array out of bounds
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = arr1[index];
    println!("The value of the element at index {} is: {}", index, element);

    //endregion Arrays
}
