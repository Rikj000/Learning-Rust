use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error>, trait object to catch any type of error

    // Attempt to open a file & handle the result with matches
    let actual_file_1 = match File::open("hello.txt") {
        Ok(opened_file) => opened_file,
        Err(error) => match error.kind() { // <= .kind() fetches enum type
            // Attempt to create the file if it's not found & handle the result
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(err) => panic!("Problem creating the file: {:?}", err)
            },
            // Handle other errors
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
    println!("Actual file 1:\n{:?}", actual_file_1);

    // Attempt to open a file & handle the result with the unwrap_or_else closure
    let actual_file_2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else { panic!("Problem opening the file: {:?}", error)  }
    });
    println!("Actual file 2:\n{:?}", actual_file_2);

    // Attempt to open a file & handle the result with unwrap
    let actual_file_3 = File::open("hello.txt").unwrap();
    println!("Actual file 3:\n{:?}", actual_file_3);

    // Attempt to open a file & handle error results with expect
    let actual_file_4 = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("Actual file 4:\n{:?}", actual_file_4);

    // Attempt to read a username from a file with error propagation
    let read_username_from_file_result_1 = read_username_from_file_1();
    println!("read_username_from_file_result_1:\n{:?}", read_username_from_file_result_1);

    // Attempt to read a username from a file with ? shortcut error propagation
    let read_username_from_file_result_2 = read_username_from_file_2();
    println!("read_username_from_file_result_2:\n{:?}", read_username_from_file_result_2);

    // Attempt to read a username from a file with chained ? shortcut error propagation
    let read_username_from_file_result_3 = read_username_from_file_3();
    println!("read_username_from_file_result_3:\n{:?}", read_username_from_file_result_3);

    // Attempt to fetch the last char of a string
    let last_char = last_char_of_first_line("Test").expect("Failed read of last char");
    println!("last_char: {}", last_char);

    let _f = File::open("hello.txt")?;
    Ok(())
}

//region read_username_from_file

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f { Ok(file) => file, Err(e) => return Err(e) };

    let mut s = String::new();
    match f.read_to_string(&mut s) { Ok(_) => Ok(s), Err(e) => Err(e) }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // Attempt to open the file, return error on fail
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> { fs::read_to_string("hello.txt") }

//endregion read_username_from_file

fn last_char_of_first_line(text: &str) -> Option<char> { text.lines().next()?.chars().last() }
