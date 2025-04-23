use rand::Rng;

fn if_elseif_else() {
    let number = rand::thread_rng().gen_range(1..=10);
    if number < 5 {
        println!("number lesser than 5");
    } else if number == 5 {
        println!("number equal to 5");
    } else {
        println!("number greater than 5");
    }
}

fn let_using_if() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    let number = if random_number > 5 { 10 } else { 5 };
    println!("10 or 5?: {number}");
    // // this wont compile because there is a type mismatch
    // let number = if random_number > 5 { 10 } else { "five" };
}

fn loops() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    loop {
        let random_guess = rand::thread_rng().gen_range(1..=10);
        if random_guess == random_number {
            println!("The random guess is equal to the random number!");
            break;
        }
        println!("trying the random guess again!");
    }
}

fn loop_returning_value() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    let random_guess = loop {
        let random_guess = rand::thread_rng().gen_range(1..=10);
        if random_guess == random_number {
            println!("The random guess is equal to the random number!");
            break random_guess; // this is how you return a value from a look. can also use return, break only exists the loop, return exist the function
        }
        println!("trying the random guess again!");
    };
    println!("random guess from outer scope: {random_guess}");
}
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whiles() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    // // could be done with a while but its slow and error prone
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("the value is: {element}");
    }

    // same code as in 'whiles()' but with 'for' using a range and 'rev()' ro reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    println!("\nif, else if and else");
    if_elseif_else();
    println!("\nlet using if");
    let_using_if();
    println!("\nloops");
    loops();
    loop_returning_value();
    loop_labels();
    println!("\nwhiles");
    whiles();
    println!("\nfor loops");
    for_loops();
}
