fn fn_parameters_return(value: i32, units: char) -> String {
    format!("value with units: {value} {units}") // this line doesnt have ';' so its an expression and not an statement. the function will return its output
}

fn statement_expression() {
    let y = {
        let x = 3; // statement, has not output
        x + 1 // expression, has output
    }; // {} is an inner scope (also an expression) that will return the output of the expression above
    println!("The value of y is: {y}");
}

fn main() {
    println!("\nFunction with arguments and return value");
    println!("{}", fn_parameters_return(5, 'h'));
    println!("\nStatements and Expressions");
    statement_expression();
}
