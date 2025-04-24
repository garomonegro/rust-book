struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_using_field_init_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,    // field init shorthand
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;

// like the unit tuple ()
fn unit_like_structs() {
    let subject = AlwaysEqual;
}

fn main() {
    //Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("User.active: {:#?}", user1.active);
    println!("User.username: {:#?}", user1.username);
    println!("User.email: {:#?}", user1.email);
    println!("User.sign_in_count: {:#?}", user1.sign_in_count);

    // Struct Update Syntax
    // this is a move not a copy, so after this user1 is invalid. Why? because of username.
    // Had we only "updated" active and sign_in_count from user1 this would have been a copy
    // because those two implement the copy trait for strings do not
    let user2 = User {
        email: String::from("another@example.com"), // changing just want we need to
        ..user1                                     // getting everything else from user1
    };
}
