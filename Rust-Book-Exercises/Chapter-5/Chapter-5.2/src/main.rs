#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

fn main() {
    // Print structs
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is: {:?}", rect1); // Print Debug
    println!("rect1 is: {:#?}", rect1); // Pretty Print Debug
    println!("The area of the rectangle is {} square pixels.", area(&rect1)); // & to borrow / retain ownership

    // Print structs, with the !dbg macro
    let scale = 2;
    let rect2 = Rectangle { width: dbg!(30 * scale), height: 50 };
    dbg!(&rect2);
}

fn area(rect: &Rectangle) -> u32 { rect.width * rect.height }
