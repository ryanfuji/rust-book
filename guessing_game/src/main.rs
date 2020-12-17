// bring io library into scope
// std library is part of Rust's prelude
use std::io;
// The `Rng` trait defines methods that random number generators implement, and this trait must be
// in scope for us to use those methods
use rand::Rng;
// Bring `Ordering` enum into scope
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // the `rand::thread_rng` function will give us the random number generator that we want to use
    // the `gen_range` method takes two numbers as arguments and generates a random number in
    // between
    // NOTE: You won't just know which traits to use and which methods and functions to call from
    // a crate. Instructions for using a crate are in each crate's documentation. You can run
    // `cargo doc --open` to read documentation
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // the `loop` keyword creats an infinite loop.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // using `let` again here is 'shadowing' the previous value of `guess` with an new one, this
        // technique is often used when you want to convert a value from one type to another type.
        // 'shadowing' lets us reuse the `guess` variable name rather than forcing us to create two
        // unique variables
        //
        // We bind `guess` to the expression `guess.trim().parse()` the `guess` in the expression refers
        // to the original `guess` that was a `String` with the user input in it. The trim method on a
        // `String` instance will remove the whitespace from the beginning an the end.
        //
        // Although `u32` can contain only numerical characters, the user must press enter to satisfy
        // `read_line`. When the user presses enter, a newline character is added to the string. For
        // Example, if the user types 5 and presses enter, `guess` looks like this: "5\n". The "\n"
        // represents the newline. The `trim()` method removes this newline character.
        //
        // `parse()` method parses a string into some kind of number. Because this method can parse a
        // variety of types, we need to tell Rust what type of number we want we do this with the
        // `let guess: u32` declaration.
        //
        // `parse()` could error, if the user inputed anything that can't be converted to a number.
        // Because it might fail, the `parse()` method returns a `Result` type, similar to the
        // `read_line()` method above. If the `Err` `Result` variant because it couldn't create a number
        // from the string, the `expect()` method will crash the program and print the message we give
        // it. If it can create a number from the string it will return the `Ok` variant of `Result`,
        // and `expect()` will return the number that we want from the `Ok` value.
        //
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Switching from an `expect()` call to a `match` express is how you generally move from
        // crashing on an error to handling the error. Remember that `parse` returns a `Result` type
        // and `Result` is an enum that has the variants `Ok` or `Err`. We're using a `match`
        // expression here as we did with the `Ordering` result of the `cmp` method.
        //
        // If `parse` is able to successfully turn the string into a number, it will return an `Ok`
        // value that contains the resulting number. That `Ok` value will match the first arm's
        // pattern, and the match expression will just return the `num` value that `parse` produced
        // and put inside the `Ok` value.
        //
        // If `parse` is not able to create a number from the string it will return the `Err` value
        // match the second arm of our `match` expression.
        // The `_` (underscore) is a catchall value, here we are saying want to match all Errors
        // The program will not crash but will instead just move on to the next iteration of the
        // loop, so basically just ignoring the incorrect value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // the `cmp()` method compares two values and can be called on anything that can be compared
        // It takes a reference to whatever you want to compare with, here it is comparing the `guess`
        // with the `secret_number`. Then it returs a variant of the `Ordering` enum
        //
        // A `match` expression is made up of many 'arms'. An arms consists of a pattern and the code
        // that should be run if the value given in the beginning of the `match` expression fits the
        // arm's pattern. Rust takes the value given to `match` and looks through each arm's pattern
        // in turn. The `match` construct and patterns are powerful features in Rust that let you express
        // a variety of situations your code might encounter and make sure that your handle them all.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // makes the program exit the loop when the user guesses the secret number correctly
                // Exiting the loop also means exiting the program, because the loop is the last
                // part of `main`
                break;
            }
        }
    }
}
