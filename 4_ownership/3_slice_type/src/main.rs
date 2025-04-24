fn string_slices() {
    let mut s = String::from("hello world");
    let word = first_word_index(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    println!("word: '{word}'");
    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    // // the code below wont compile because because .clear will use a mutable reference to s2.
    // // and word in the line after that is a immutable reference to part of the same string
    // // (we saw this in 2_references_borrowing)
    // let mut s2 = String::from("hello world");
    // let word = first_word(&s2);
    // s2.clear(); // error!
    // println!("the first word is: {word}");

    let s2 = String::from("hello world");
    let word = first_word(&s2);
    println!("the first word is: {word}");

    // string slice
    // Case 1
    let s = String::from("hello world");
    let hello = &s[0..5]; // this range doesnt include the 5, so it wont have the space in it
    let world = &s[6..11];
    println!("hello: '{hello}'");
    println!("world: '{world}'");
    // Case 2
    let s = String::from("hello");
    let len = s.len();
    // the two below are the same
    let slice = &s[3..len];
    println!("slice: '{slice}'");
    let slice = &s[3..];
    println!("slice: '{slice}'");
    // case 3
    let s = String::from("hello");
    let len = s.len();
    // the two below are the same
    let slice = &s[0..len];
    println!("slice: '{slice}'");
    let slice = &s[..];
    println!("slice: '{slice}'");

    let s = "Hello, world!"; // string literals are a &str
    println!("this is a string literal (&str) an immutable references to a string slice: {s}")
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
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

fn array_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn main() {
    string_slices();
    array_slices();
}
