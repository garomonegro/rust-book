// rand is a library 'crate', this compiled program is a binary crate
// crates are pulled from the 'crates.io' registry
// Rng is a trait that defines methods that random numbers generators can implement
use rand::Rng;
// the standard library 'std', the set is call 'prelude': https://doc.rust-lang.org/std/prelude/index.html
// io is a module
// cmp is a module and Ordering is an enum with variants Less, Greater and Equal
use std::{cmp::Ordering, io};

fn main() {
    // 'println!' is a macro, macros are suffixed with '!'.
    // https://doc.rust-lang.org/book/ch20-05-macros.html
    println!("Guess the number!");
    // thread_rng() returns a random number generator
    // gen_range() is a method of said generator, defined by the Rng trait
    // 1..=100 is an inclusive (includes the bounds) range in the format start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
        // 'let' creates a new variable.
        // 'mut' makes the variable mutable. variables are immutable by default.
        // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
        // 'String::new()' is a new instance of String, a growable UTF-8 encoded bit of text.
        // '::' indicates that new is an associated function of the String type. An associated function is a function implemented on a type.
        // https://doc.rust-lang.org/std/string/struct.String.html
        let mut guess = String::new();

        // 'stdin()' function from module 'io' that returns an instance of 'std::io::Stdin'.
        // Stdin is a type that represents a handle to the standard input for the terminal.
        // This could have been std::io::stdin() if 'use std::io;' was not used.
        io::stdin()
            // 'read_line' is a method of Stdin
            // '&mut guess' here '&' indicates a reference, also immutable by default, hence 'mut' is needed such that read_line can mutate it.
            // references let multiple parts of code access data without coping it into memory multiple times
            // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
            .read_line(&mut guess)
            // read_line returns a 'Result' value which is an enum, each enum state is a 'variant'
            // https://doc.rust-lang.org/book/ch06-00-enums.html
            // The purpose of Result types is to encode error-handling information
            // Results variants are Ok and Err
            // expect will crash the program and display the given string is Result is Err
            // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
            .expect("Failed to read line");
        if guess.trim().to_lowercase() == "quit" {
            println!("You quit, loser!");
            break;
        }
        // guess here is 'shadowing' the original guess, allowing us to use the same name
        // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
        // guess.trim() removes any leading or trailing white spaces. Enter pressed after the number is typed adds a new line, e.g. '34\n'.
        // the ':' is an annotation onf the variables type
        // .parse converts the string to a number
        // // .expect will return back said number in ok if Results is of the ok variant and fail and display the message if its Err.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Thats not a number!");
                println!("Type 'quit' if you are a loser!");
                continue;
            }
        };
        // Here {} is a placeholder, it could have been 'println!("You guessed {guess}");'
        println!("You guessed {}", guess);
        // cmp is a method, can be call on anything that can be compared. Returns the variant of Ordering based on the comparison
        // match is an expression made of 'arms', an arm consists of a pattern to match against and the code that will run
        // https://doc.rust-lang.org/book/ch06-00-enums.html
        // https://doc.rust-lang.org/book/ch19-00-patterns.html
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
