fn main() {
    if_expressions();
    multiple_conditions();
    using_if_in_a_let_statement();
    repeating_with_loop();
    return_values_from_loops();
    conditional_while_loops();
    looping_through_collection_with_for();
    looping_range_with_for();
}

fn if_expressions() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The condition must be a bool. If the condition is not a bool, we get a compile error
    // the code below will not work
    // if number == 3 {
    //     // number {
    //     println!("number was three");
    // }
}

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Because `if` is an expression, we can use it on the right side of a `let` statement. The `number`
// variable will be bound by to a value based the outcome of the `if` expression
fn using_if_in_a_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is: {}", number);

    // Remember that blocks of code evaluate to the last expression in them, and numbers by
    // themselves are also expressions. In this case, the value of the whole if expression depends
    // on which block of code executes. This means the values that have the potential to be results
    // from each arm of the `if` must be of the same type.
    // below will not work
    // let number = if condition { 5 } else { "six" };
}

fn repeating_with_loop() {
    // `loop` tells rust to execute block of code over and over again forever or until you explicity
    // tell it to stop
    loop {
        println!("again!");
        break;
    }
}

// One of the uses of a `loop` is to retry an operation you know might fail, such as checking
// whether a thread has completed its job. However, you might need to pass the result of that
// operation to the rest of your code. To do this, you can add the value you want returned after
// the `break` expression you use to stop the loop; that value will be returned out of the loop so
// you can use it
fn return_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the loop is: {}", result);
}

// It's often useful for a program to evaluate a condition within a loop. While the condition is true
// the loop runs. When condition ceases to be true, the program calls `break`, stopping the loop.
// This loop type could be implemented using a combination of `loop`, `if`, `else` and `break`, but
// another way is the `while` loop: the program below loops 3 times, counting down each time, and then,
// after the loop, it prints another message and exits.
fn conditional_while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!!");
}

fn looping_through_collection_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// Another way is to a `Range`, which is a type provided by the standard library that generates
// all the numbers in sequence starting from one number and ending before another number.
fn looping_range_with_for() {
    // `rev()` means go in reverse order
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
