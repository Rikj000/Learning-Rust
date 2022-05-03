// Implement a (debuggable) Rectangle struct
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

// Implement methods for the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn width(&self) -> bool { self.width > 0 }
    fn height(&self) -> bool { self.height > 0 }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Custom square constructor (Associated function)
    fn square(size: u32) -> Rectangle { Rectangle { width: size, height: size } }
}

fn main() {
    // Initialize some rectangles
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square = Rectangle::square(3);

    // Utilize the new methods
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() { println!("The rectangle has a nonzero width, it is {}", rect1.width); }
    if rect1.height() { println!("The rectangle has a nonzero height, it is {}", rect1.height); }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square height: {}, width: {}", square.height, square.width);
}
