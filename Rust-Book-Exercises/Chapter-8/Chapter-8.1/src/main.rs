enum SpreadSheetCell { Int(i32), Float(f64), Text(String) }

fn main() {
    // Initialize some vectors
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3]; // Macro to initialize a vector with values

    // Updating a vector v3
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // Fetch the third element of the v3 vector without error handling
    let third: &i32 = &v3[2];
    println!("The third element of vector v3 is {}", third);

    // Match the third element of the v3 vector with error handling
    match v3.get(2) {
        Some(third_element) => println!("The third element of vector v3 is {}", third_element),
        None => println!("There is no third element in vector v3")
    }

    // Iterate over the values in the v3 vector
    for i in &v3 { println!("v3 element: {}", i); }

    // Iterate over a vector v4 and mutate it's values
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 { *i += 50; } // * = dereference operator to get the value in i

    // Vector + enum to store multiple types
    let _row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12)
    ];

} // <- _v1, _v2, v3, v4 and _row will all go out of scope and be freed here
