fn copy_vs_move() {
    let x = 5; // x has a fixed size, so it will be placed in the stack
    let y = x; // this is a copy, here the value 5 is copied to the next place in the stack

    // more info: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
    let s1 = String::from("hello"); // "hello" will go in the heap because Strings do not have a fixed size, the stack will have s1 as a pointer to the heap + some other information
    let s2 = s1; // this is a move, the stack data will be copied and s1 will be invalidated. copying the data in the stack means coping the pointer to the heap, not copying the data in the heap

    let s1 = String::from("hello");
    let s2 = s1.clone(); // this will be a copy of the heap data, similar to a deep copy. its an expensive operation compare the move or copy of stack data
}

fn functions_and_ownership() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_ownership() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn main() {
    copy_vs_move();
    functions_and_ownership();
    return_values_and_ownership();
}
