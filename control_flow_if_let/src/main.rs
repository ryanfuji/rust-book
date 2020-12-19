// The `if let` syntax lets you combine `if` and `let` into a less verbose why to andle values that
// match one pattern while ignoring the rest. The code below matches on an `Option<u8>` value and
// only exectues code if the value is 3
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

// Instead of above, we could write this in a shorter way using `if let`. Below code behaves the
// same as the `match` above
#[allow(dead_code)]
fn if_let() {
    let some_u8_value = Some(0u8);
    // The syntax `if let` takes a pattern and an expression separated by an equal sign. It works
    // the same way as a `match`, where the expression is given to the `match` and the pattern is
    // its first arm. Using this syntax means less typing, less indentation, and less boilerplate
    // code. However, you lose the exhaustive checking that `match` enforces. Choosing between the
    // `match` and the `if let` depends on what you're doing in a particular situation and whether
    // gaining conciseness is an appropriate trade-off for losing the exhaustive checking.
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// We can include an `else` with an `if let`. The block of code that goes with the `else` is the
// same as the block of code that would go with the `_` case in the `match` expression that is
// equivalent to the `if let` and `else`.
//
// Using the `Coin` enum definition in the match_operator crate, where the `Quarter` variant also
// held a `UsState` value. If we wanted to count all the non-quarter coins we see while also
// announcing the state of the quarters, we could that with a `match` expression like below
#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // rest of states
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn match_quarters() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

// We could also use an `if let` and `else expression like this
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn if_let_else_quarters() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
