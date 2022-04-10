fn main() {
    print_labeled_measurement(5, 'h');

    //region Scope Block
    let x = {
        // Inner-scope
        let temp = 3;
        // Return
        temp + 1
    };
    println!("The outer-scope value of x is: {}", x);
    //endregion Scope Block

    let y = five();
    println!("The value of y is: {}", y);

    let z = plus_one(5);
    println!("The value of z is: {}", z);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 { x + 1 }
