fn main() {
    //region First/Second Word
    let mut s = String::from("Hello world I'm learning");
    let word1 = first_word(&s);
    println!("Word 1: {}", word1);
    let word2 = second_word(&s);
    println!("Word 2: {}", word2);
    //endregion First/Second Word

    //region String Slices
    let hello = &s[..5];
    println!("Hello: {}", hello);
    let world = &s[6..11];
    println!("World: {}", world);
    let slice_start_to_end = &s[..];
    println!("Slice start to end: {}", slice_start_to_end);

    s.clear(); // this empties the String, making it equal to ""
    //endregion String Slices

    //region String, str, string literals

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    //endregion String, str, string literals

    //region Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    //endregion Array slices
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start: usize = 0;
    let mut end: usize = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 { start = i + 1; }
            else if end == s.len() {
                end = i;
                break;
            }
        }
    }

    &s[start..end]
}