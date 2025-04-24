#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// anything within this implementation block will be associated with Rectangle
// all functions within this block are "associated functions" but not all are methods
impl Rectangle {
    // first argument of a method is self
    // &self is shorthand for self: &self
    // here we take a reference & indicating this methos borrows self immutably
    // we could borrow self mutably or take ownership of it as well
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // another method
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width || self.width >= other.height)
            && (self.height >= other.width || self.height >= other.height)
    }
    // not a method but an associated function
    // Self (not self) here is an alias for Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// you can have multiple implementation blocks
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // notice that this is not '&rect1.area()' because of a rust feature called "Automatic Referencing and Dereferencing"
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(25);
    println!("square: {:?}", square);

    let rect4 = Rectangle::new(50, 30);
    println!("react4: {:#?}", rect4);
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}
