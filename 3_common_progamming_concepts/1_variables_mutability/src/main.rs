fn mutability() {
    // this will compile and run
    let mut x = 5;
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    // This should not compile because the code tries to mutate x but it was not declared as mutable.

    //     let x = 5;
    //     println!("x: {x}");
    //     x = 6;
    //     println!("x: {x}");
}

// constants are always immutable, hence you cant use 'mut' with them
// they cant hold values that can only be computed at runtime, unlike variables
// the type must be annotated, unlike variable where the type can be inferred by the compiler
// naming convention is to use uppercase with underscores
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// you must use 'let' every time for shadowing to work, else code does not compile like in mutability_bad
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x in inner scope: {x}");
    }
    println!("x in main scope: {x}");
}

fn shadowing_not_mutability() {
    let spaces = "     ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // This wont compile because you cant change the type of a mutable variable

    // let mut spaces = "     ";
    // println!("spaces: {spaces}");
    // spaces = spaces.len();
    // println!("spaces: {spaces}");
}

fn main() {
    println!("\nmutability");
    mutability();

    println!("\nconstants");
    println!("3 hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    println!("\nshadowing");
    shadowing();
    shadowing_not_mutability();
}
