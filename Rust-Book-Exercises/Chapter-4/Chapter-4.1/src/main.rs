fn main() {
    stack_string_description();
    heap_string_example();
    heap_move_example();
    heap_clone_example();
    stack_clone_example();

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it's okay to still use x afterward

    let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let _s2 = String::from("hello"); // s2 comes into scope
    let _s3 = takes_and_gives_back(_s2);  // s2 is moved into takes_and_gives_back,
    // which also moves its return value into s3

    let (s2, len) = calculate_length(_s1);
    println!("The length of '{}' is {}.", s2, len);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
// s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// This function gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

//region Stack & Heap Examples
fn stack_string_description() { // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward
    println!("{}", s) // do stuff with s
} // this scope is now over, and s is no longer valid

fn heap_string_example() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn heap_move_example() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}

fn heap_clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_clone_example() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
//endregion Stack & Heap Examples