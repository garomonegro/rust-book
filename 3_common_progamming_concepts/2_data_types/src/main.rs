fn tuples() {
    // tuples have a fixed length but they can hold diff types
    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    println!("tuple: {:?}", tup);
    let tup = (-500, 6.4, 1);
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");
    // The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.
}

// arrays also have a fixed length but they hold the same type
// arrays are allocated in the stack
fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:#?}", a);
    let a = [3; 5];
    println!("a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    // Invalid Array Element Access
    // // this wont compile, but it would also failed if it were to happen in runtime
    // println!("a[5]: {}", a[5]);
    // This is an example of Rust’s memory safety principles.
    // In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed.
}

fn main() {
    println!("\ntuples");
    tuples();
    println!("\narrays");
    arrays();
}
