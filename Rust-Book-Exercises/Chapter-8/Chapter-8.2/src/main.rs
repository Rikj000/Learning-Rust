fn main() {
    // Initialize a new empty string and push some data onto it
    let mut s1 = String::new();
    s1.push_str("initial contents 1");
    println!("s1: {}", s1);

    // Initialize a string with data
    let data = "initial contents 2";
    let s2 = data.to_string();
    println!("s2: {}", s2);

    // The method also works on a literal directly:
    let s3 = "initial contents 3".to_string();
    println!("s3: {}", s3);

    // Same as above
    let s4 = String::from("initial contents 4");
    println!("s4: {}", s4);

    // All valid UTF-8 encoded string values
    let _hello_a = String::from("السلام عليكم");
    let _hello_b = String::from("Dobrý den");
    let _hello_c = String::from("Hello");
    let _hello_d = String::from("שָׁלוֹם");
    let _hello_e = String::from("नमस्ते");
    let _hello_f = String::from("こんにちは");
    let _hello_g = String::from("안녕하세요");
    let _hello_h = String::from("你好");
    let _hello_i = String::from("Olá");
    let _hello_j = String::from("Здравствуйте");
    let _hello_k = String::from("Hola");

    // Concatenation with the + operator
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // note s5 has been moved here and can no longer be used
    println!("s7: {}", s7);

    // Concatenation with the format! macro
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("s11: {}", s11);

    // Slicing Strings
    let hello = "Здравствуйте"; // each of these special characters is 2 bytes
    let s12 = &hello[0..4];
    println!("s12: {}", s12);

    // Iterating over Strings
    for c in "नमस्ते".chars() { println!("Char: {}", c); }
    for b in "नमस्ते".bytes() { println!("Byte: {}", b); }
}
