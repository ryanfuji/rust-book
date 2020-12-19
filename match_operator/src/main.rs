// Rust has an extremely powerful control flow operator called `match` that allows you to compare
// a value against a series of patterns then execute code based on which pattern matches. Patterns
// can be made up of literal values, variable names, wildcards, and others. The power of `match`
// comes from the expressiveness of the patterns and the fact that the compiler confirms that all
// possibles cases are handled
//
// Think of a `match` expression as being like a coin-sorting machine: coins slide down a track with
// variously sized hols along it, and each coin falls through the first hole it encounters that it
// fits into. In the same way, values go through each pattern in a `match`, and at the first
// pattern the value "fits", the value falls into the associated block of code to be used during
// execution
//
// We can write a function that can take an unknown United States coin and, in a similar way as the
// counting machine, determine which coin it is and return its value in cents
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    // first we list the `match` keyword followed by an expression, `coin` in this case
    match coin {
        // Next are the `match` arms. An arm has two parts: a pattern and some code. This first arm
        // here is a pattern that is the value `Coin::Penny` and then the `=>` operator that
        // separates the pattern and the code to run. Here the code is just the value `1`
        Coin::Penny => 1,
        // each arm is separated by a comma
        Coin::Nickel => 5,
        // When the `match` expression exectues, it compares the resulting value against the pattern
        // of each arm, in order. If a pattern matches the value, the code associated with that pattern
        // is executed. If the pattern doesn't match the value, it moves on to the next arm
        Coin::Dime => 10,
        // The code associated with each arm is an expression, and the resulting value of the
        // expression in the matching arm is the value that gets return for the entire `match`
        // expression
        Coin::Quarter => 25,
    }
}

#[allow(dead_code)]
fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        // Curly brackets typically aren't used if the match arem code is shart as it is in the
        // example above where each arm just returns a value. If you want to run multiple lines of
        // code in a match arm, you can use curly brackets, like below, which will printe "Luck Penny!"
        // every time the method was called with a `Coin::Penny` but would still return the last
        // value of the block `1`
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Another useful feature of match arms is that they can bind to the parts of the values that match
// the pattern. This is how we can extract values out of enum variants
//
// As an example, let's change on of the our enum variatns to hold data inside it. From 1999 through
// 2008, the United States minted quarters with different designs for each of the 50 states on one
// side. No other coins got state designs, so only quarters have this extra value. We can add this
// information to `enum` by changing the `Quarter` variant to include a `UsState` value stored inside
#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // and 48 others
}

#[allow(dead_code)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Let's imagine a friend of ours is trying to collect all 50 state quarters. While we sort our loose
// change by coin type, we'll also call out the name of the state associated with each quarter so if
// it's one our friend doesn't have, they can add it to their collection
fn value_in_cents_state_quarter_added(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        // we add a variable called `state` to the pattern that matches values of the vairant
        // `Coin::Quarter`. When a `Coin::Quarter` matches, the `state` variable will bind to the
        // value of that quarter's state. Then we can use `state` in the code for that arm
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // If we were to call
    let mut value = 0;
    value += value_in_cents_state_quarter_added(Coin2::Quarter(UsState::Alaska));
    value += value_in_cents_state_quarter_added(Coin2::Dime);
    println!("We have {} cents in coins.", &value);
}
