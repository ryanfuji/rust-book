// To panic! or Not to panic!
//
// So how do you decide when you should call `panic!` and and when you should return `Result`? When
// code panics, there's no way to recover. You could call `panic!` for any error situation, whether
// there's a possible way to recover or not, but then you're making the decision on behalf of the
// code calling your that a situation is unrecoverable. When you choose to return a `Result` value,
// you give the calling code options rather than making decisions for it. The calling code could
// choose to attempt to recover in a way that's appropriate for its siutation, or it could decide
// that an `Err` value in this case is unrecoverable, so it can call `panic!` and turn your
// recoverable error into an unrecoverable one. Therefor, returning `Result` is a good default choice
// when you're defining a function that might fail.
//
// In rare situations, it's more appropriate to write code that panics instead of return a `Result`.
// Let's explore why it's appropriate to panic in examples, prototype code, and tests. Then we'll
// discuss situations in which the compiler can't tell that failure is possible, but you as a human
// can.
//
// Examples, Prototype Code, and Tests
//
// When you're writing an example to illustrate some concept, having robust error-handling code in
// the example as well can make the example less clear. In examples, it's understood that a call to
// a method like `unwrap` that could panic is meant as a placholder for the way you'd want your
// application to handle errors, which can differ based on what the rest of your code is doing.
//
// Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before you're ready
// to decidehow to handle error. They leave clear markers in your code for when you're ready to
// make your program more robust.
//
// If a method call fails a test, you'd want the whole test to fail, even if that method isn't the
// functionality under test. Because `panic!` is how a test is marked as a faulture, calling `unwrap`
// or `expect` is exactly what should happen.
//
// Cases in Which You Have More Information that the Compiler
//
// It would also be appropriate to call `unwrap` when you have some other logic that ensures the
// `Result` will have an `Ok` value, but the logic isn't something the compiler understands. You'll
// still have a `Result` value that you need to handle: whatever operation you're calling still has
// the possibility of failing in general, even though it's logically impossible in your particular
// situation. If you can ensure by manually inspecting the code that you'll never have an `Err`
// variant, it's perfectly acceptable to call `unwrap`. Here is an example.
use std::net::IpAddr;

fn never_fail_acceptable_unwrap_call() {
    // We're creating an `IpAddr` instance by parsing a hardcoded string. We can see that `127.0.0.1`
    // is a valid IP address, it's acceptable to call `unwrap` here. However, having a hardcoded,
    // valid string doesn't change the return type of the `parse` method: we still get a `Result`
    // value, and the compiler will still make us handle the `Result` as if the `Err` variant is a
    // possibility because the compiler isn't smart enough to see that this string is always a valid
    // IP address. If the IP address string came from a user rather than being hardcoded into the
    // program and there did have a possiblity of failure, we'd definitely want to handle the `Result`
    // in a more robust way instead.
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{:?}", home);
}

// Guidelines for Error Handling
//
// It's advisable to have your code panic when it's possible that your code could end up in a bad
// state. In this context, a "bad state" is when some assumption, guarantee, contract, or invariant
// has been broken, such as when invalid values, contradictory values, or missing values are passed
// to your code--plus one or more of the following...
// - The bad state is not something that's expected to happen occasionally
// - Your code after this point needs to rely on not being in this bad state
// - There's not a good way to encode this information in the types you use
//
// If someone calls your code and passes in values that don't make sense, the best choice might be
// to call `panic!` and alert the persion using your library to the bug in their code so they can it
// during development. Similarly, `panic!` is often appropriate if you're calling external code that
// is out of your control and it returns an invalid state that you have no way of fixing.
//
// When your code performs operations on values, your code should verify the values are valid first
// panic if the values aren't valid. This is mostly for safety reasons: attempting to operate on
// invalid data can expose your code to vulnerabilities. This is the main reason the standard library
// will call `panic!` if you attempt an out-of-bounds memory access: trying to access memory that
// doesn't belong to the current data structure is a common security problem. Functions often have
// "contracts": their behavior is only guaranteed if the inputs meet particular requirements.
// Panicking when the contract is violated makes sense because a contract violation always indicates
// a caller-side bug and it's not a kind of error you want calling code to have to explicity handle.
// In fact, there's no reasonable way for calling code to recover; the calling programmers need to
// fix the code. Contracts for a function, especially when a violation will cause a pnaic, should
// be explained in the API documentation for the function.
//
// However, having lots of error check in all of your functions would be verbose and annoying.
// Fortunately, you can use Rust's type system (and thus the type checking the compiler does) to
// do many of the checks for you. If your function has a particular type as a parameter, you can
// proceed with your code's logic knowing that the compiler ahs already ensured you have a valid
// value. For example, if you have a type rather than an `Option`, your program expects to have
// "something" rather "nothing". Your code then doesn't have to handle two cases for the `Some` and
// `None` variants: it will only have one case for definitely having a value. Code trying to pass
// nothing to your function won't even compile, so your function doesn't have to check for that case
// at runtime. Another example is using an unsigned integer type such as u32, which ensures the
// paramter is never negative.
//
// Creating Custom Types for Validation
//
// Let's take the idea of using Rust's type system to ensure we have a valid value one step further
// and look at creating a custom type for validation. Recall from the guessing game crate in which
// the code asked the user to guess a number between 1 and 100. We never validated that the user's
// guess was between those numbers before checking it against our secret number; we only validated
// that the guess was positive. In this case, the consequences were not dire: our output of "Too High",
// or "Too Low" would still be corrrect. But it would be a useful enhancement to guide the user
// toward valid guesses and have different behavior when a user guesses a number that's out of range
// versus when a users types, for example, letters instead
//
// See example line 81 main.rs, guessing_games crate
//
// However, that solution is not an ideal solution: if was absolutely critical that the program only
// operate on values between 1 and 100, and it had many functions with this requirement, having a
// check like this in every function would be tedious (and might impact performance).
//
// Instead, we can make a new type and put the validations in a function to create an instance of
// the type rather than repeating the validations everywhere. That waym it's safe for functions to
// use the new type in their signatures and confidently use the values they receive. Below shows
// one way to define a `Guess` type that will only create an instance of `Guess` if the `new`
// function receives a value between 1 and 100.

// First, we define a struct named `Guess` that has a field named `value` that holds an i32. This is
// where the number will be stored
pub struct Guess {
    value: i32,
}

impl Guess {
    // Then we implement an associated function named `new` on `Guess` that creates instances of
    // `Guess` values
    pub fn new(value: i32) -> Guess {
        // we test `value` to make sure it's between 1 and 100.
        if value < 1 || value > 100 {
            // If `value` doens't pass the test, we make a `panic!` call, which will alert the
            // programmer who is writting the calling code that we have a bug that needs a fix,
            // because creating a `Guess` with a `value` outside this range would violate the contract
            // that `Guess::new` is relying on
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    // Next, we implement a method named `value` that borrows `self`, doesn't have any other
    // parameters, and returns a i32. This kind of method is sometimes called a "getter", because
    // its purpose is to get some data from it fields and return it. This public method is necessary
    // because the `value` field of the `Guess` struct is private. It's important that the `value`
    // field be private so code using the `Guess` struct is not allow to set `value` directly: code
    // outside the module must use the `Guess::new` function to create an instance of `Guess`,
    // thereby ensuring there's no way for a `Guess` to have a `value` that hasn't been checked
    // by the conditions in the `Guess::new` function
    pub fn value(&self) -> i32 {
        self.value
    }

    // A function that has a paramter or returns only numbers between 1 and 100 could then declare
    // in its signature that takes or returns a `Guess` rather than an i32 and wouldn't need to do
    // and additional check in its body.
}

fn main() {
    never_fail_acceptable_unwrap_call();
}
