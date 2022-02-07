// https://doc.rust-lang.org/book/ch04-03-slices.html

fn main() {
    let s = String::from("This is a word.");
    println!("Hello, world! {}", first_word(&s));

    let my_string = String::from("hello world");

    // 'firstword works on slices of strings
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    //works on ref to Strings, which are equiv to whole slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // string literals are string slices
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal);

    // int slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}