// In the match_operator crate we wanted to the inner `T` value out of `Some` case when using
// `Option<T>`; we can also handle `Option<T>` using `match` as we did with the `Coin` enum.
// Instead of comparing coins, we'll compare the variants of `Option<T>`, but the way that `match`
// expression works is the same.

// Let's say we want to write a function takes an `Option<i32>` and, if there's a value inside, adds
// 1 to that value. If there isn't a value inside, the function should return the `None` value not
// attempt to perform any operations
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn call_plus_one() {
    let five = Some(5);
    // When we call this the variable `x` in the body of `plus_one` will have the value `Some(5)`
    // so it will match the pattern `Some(i)` and the `Some(i + 1)` code will execute and return
    // `Some(6)`
    let six = plus_one(five);

    let none = plus_one(None);
}

// Rust also has a pattern we can use when we don't want to list all possible values. For example, a
// u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we
// don't want to have to list out all those numbers. We can use the `_` (underscore) special pattern
// instead
fn underscore_pattern(value: u8) {
    match value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    underscore_pattern(1);
    underscore_pattern(200);
    underscore_pattern(5);
}
