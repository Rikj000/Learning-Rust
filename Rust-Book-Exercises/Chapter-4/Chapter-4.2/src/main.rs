fn main() {
    // Immutable reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    let mut s2 = String::from("hello2");
    change(&mut s2);
    println!("s2 value is {}", s2);

    // Multiple mutable references using a scope
    let mut s3 = String::from("hello");
    {
        let _r1 = &mut s3;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s3;

    // Immutable & mutable references combined
    let mut s = String::from("hello");
    let t1 = &s; // no problem
    let t2 = &s; // no problem
    println!("{} and {}", t1, t2);
    // variables t1 and t2 will not be used after this point
    let t3 = &mut s; // no problem
    println!("{}", t3);

    // Dangle/No-dangle
    let no_dangle = no_dangle();
    println!("{}", no_dangle);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope.
// But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
