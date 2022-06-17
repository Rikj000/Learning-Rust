fn main() {
    // Fetch the largest number in the list
    let number_list1 = vec![34, 50, 25, 100, 65];
    let mut largest1 = number_list1[0];

    for number in number_list1 {
        if number > largest1 { largest1 = number; }
    }

    let result1 = largest1;
    println!("The largest number is {}", result1);

    // Fetch the largest number in the list through code abstraction to a new function
    let number_list2 = vec![34, 50, 25, 100, 65];
    let result2 = largest(&number_list2);
    println!("The largest number is {}", result2);

    let number_list3 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result3 = largest(&number_list3);
    println!("The largest number is {}", result3);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest { largest = item; }
    }

    largest
}