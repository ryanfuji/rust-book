fn main() {
    println!("Hello, world!");
    // Even though this function is defined after the `main` function, Rust doesn't care
    another_function();
    expressions();
    call_functions_with_return_values();
}

fn another_function() {
    println!("Another function.");
    function_parameters(5);
}

// Functions can be defined to have 'parameters', which are special variables that a part of a
// function's signature. When a function has parameters, you can provide it with concrete values
// for those parameters. Technically, the concrete values are callec 'arguments' but the two terms
// can be used interchangeably.
fn function_parameters(x: i32) {
    println!("The value of x is: {}", x);
}

// Function Bodies Contain Statements and Expressions
//
// Function bodes are made up of a series of statements optionally ending in an expression. So far,
// we've only covered bunction without an ending expression, but you have seen an expression as
// part of a statement.
//
// Because rust is an expression-based language, this is an important distinction to understand.
fn _statements() {
    // creating a variable and assigning a vlue to it with `let` keyword is a statement
    let _y = 6;
}

// Function definitions are also statements; the entire `_statements_and_expressions()` definition
// is a statement in itself

// Statements do not return values. Therefore you can't assign a `let` statement to another variable,
// as the following code to do, we get a error if remove the comment on line in the function
// definitions
//
// The `let y = 6` statement does not return a value, so there isn't anything for `x` to bind to
// In Rust assignments don't return the values of the assignment
fn _statements2() {
    // let x = (let y = 6);
}

// Expressions evaluate to something and make up most of the rest of the code that you'll write in
// Rust.
//
// Consider a simple math operation, like `5 + 6`, which is an expression that evaluates to the value
// 11. Expressions can be part of statements...
fn expressions() {
    let x = 5;
    // in the block below, it evaluates to 4. That value gets bound to `y` as part of the `let`
    // statement
    let y = {
        let x = 3;
        // this line does not end with a semicolon. Expressions do not include ending semicolons.
        // If you add a semicolon to the end of the expression, you turn it into a statement, which
        // will then not return a value.
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn call_functions_with_return_values() {
    let x = functions_with_return_values();
    println!("The value of x is: {}", x);
}

// functions can return vales to the code that calls them. We don't name return values, but we
// do declare their type after an arrow (->). You can return early from a function using `return`
// keyword and specifying a value, but most functions return the last expression implicity like it
// does below
fn functions_with_return_values() -> i32 {
    5
}
