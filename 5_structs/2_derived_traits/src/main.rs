#[derive(Debug)] // this is how we opt in and derive the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn printing_structs_with_derive_trait() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// dbg! takes ownership, prints to stderr (as opposed to stdout like println) and returns that ownership back
// (so it can be use directly in ':' or '=' assignments)
fn using_dbg_instead_of_println() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
fn main() {
    printing_structs_with_derive_trait();
    using_dbg_instead_of_println();
}
