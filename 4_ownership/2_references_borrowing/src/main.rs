fn functions_move_vs_borrow() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length_with_move(s1);
    println!("The length of '{s2}' is {len}.");

    let len = calculate_length_with_borrow(&s2);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length_with_move(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_with_borrow(s: &String) -> usize {
    s.len()
}

fn functions_mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // // you cant borrow a mutable variable more than once at a time, the code bellow wont compile
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // is does not mean its a second borrow of a mutable variable because its not at the same time
    change2(&mut s);
    println!("{s}");

    // this compiles because the borrows are not at the same time
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // // you cant have a reference and a mutable reference at the same time, the code below wont compile
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // But you can have multiple immutable references
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, and {}", r1, r2);

    // the code below does compile because the immutable references are not use after the mutable reference is created
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");
    // println!("{r1} and {r2}");// this would make it not compile

    // let reference_to_nothing = dangle();
}

// // this code would not compile because &s is a dangling references: a pointer to a variable that wont exist
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change2(some_string: &mut String) {
    some_string.push_str("!");
}

fn main() {
    functions_move_vs_borrow();
    functions_mutable_reference();
}
