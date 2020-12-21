// Error Handling
//
// Rust's commitment to reliability extends to error handling. Errors are a fact of life in software
// so Rust has a number of features for handling situations in which something goes wrong. In many
// cases, Rust requires you to acknowledge the possibility of an error and take some action before
// your code will compile. This requirement makes your program more robust by ensuring that you'll
// discover errors and handle appropriately before you've deployed your code to production.
//
// Rust groups errors into two major categories: "recoverable" and "unrecoverable" errors. For a
// recoverable error, such as a file not found error, it's reasonable to report the problem to the
// user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to
// access a location beyond the end of an array.
//
// Most languages don't distinguish between these two kinds of errors and handle both in the same
// way, using mechanisms such as exceptions. Rust doesn't have exceptions. Instead, it has the type
// `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the
// program encounters an unrecoverable error.

// Unrecoverable Errors with `panic!`
//
// Sometimes, bad things happen in your code, and there's nothing you can do ab out it. In these
// cases Rust has the `panic!` macro. When the `panic!` macro executes, your program will print a
// failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a
// bug of some kind has been detected and it's not clear to the programmer how to handle the error.
//
// Unwinding the Stack or Aborting in Response to a Panic
//
// By default, when a panic occurs, the program starts "unwinding", which means Rust walks back up
// the stack and cleans up the data from each function it encounters. But this walking back and
// cleanup is a lot of work. the alternative is to immediately "abort", which ends the program
// without cleaning up. Memory that the program was using will then need to be cleaned up by the
// OS. If in your project you need make the resulting binary as small as possible, you can switch
// from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate
// `[profile]` sections in your Cargo.toml file. For example, if you want to abort on panic in
// release mode, add this:
//
//              `[profile.release]`
//              `panic = 'abort'`

#[allow(dead_code)]
fn call_panic() {
    // The call to `panic!` causes the error message to be displayed as well as where
    panic!("crash and burn");
}

// Recoverable Errors with `Result`
//
// Most errors aren't serious enough to require the program to stop entireley. Sometimes, when a
// a function fails, it's for a reason that you can easily interpret and respond to. For example,
// if you try to open a fiel and that operation fales because the file doesn't exist, you might want
// to create the file instead of terminating the process

// Remember the `Result` type enum from the guessing_game crate, is defined as...
//
//              enum Result<T, E> {
//                  Ok(T),
//                  Err(E),
//              }
//
// The `T` and `E` are generic type parameters. `T` represents the type of the value that will be
// returned in a success case within the `Ok` variant, and `E` represents the type of error that
// will be returned in a failure case with the `Err` variant. Because `Result` has these generic
// type parameters, we can use the `Result` type and the function that the standard library has
// defined on it in many different situations where the successful value and error value we want to
// return may differ.
//
// Let's call a function that returns a `Result` value because the function could fail.
use std::fs::File;

#[allow(dead_code)]
#[allow(unused_variables)]
fn call_function_that_could_fail() {
    // How do we know `File::open` returns a result? If we give `f` a type annotation that we know
    // is not the return type of the function and then try to compile the code, the compiler will
    // tell us that the types don't match. The error message will then tell us what the type of `f`
    // is.
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// The code above will panic no matter why `File::open` failed. What we want to do instead is take
// different action for different failure reason: if `File::open` failed because the file doesn't
// exist, we want to dcreate the file and return the handle to the new file. If `File::open` failed
// for any other reason--for example, because we didn't have permission to open the file--we still
// want the code to panic in the same way as it did above
use std::io::ErrorKind;

// This function adds an inner `match` expression
#[allow(dead_code)]
#[allow(unused_variables)]
fn match_on_different_errors() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // the type of the value that `File::open` returns inside the `Err` variant is `io::Error`,
        // which is a struct provided by the standard library. This struct has a method `kind` that
        // we can call to get an `io::ErrorKind` value
        Err(error) => match error.kind() {
            // the enum `io::ErrorKind` is provided by the standard library and has variants
            // representing the different kinds of errors that might result form an `io` operation.
            // The variant we want to use is `ErrorKind::NotFound`, which indicates that the file
            // we're trying to open doesn't exist yet. So we match on `f`, but we also have an inner
            // match on `error.kind()` The condition we want to check in the above enclosing match,
            // is whether the value returned by `error.kind()` is the `NotFound` variant of the
            // `ErrorKind` enum. If it is, we try to create the file with `File::create`. However,
            // because `File::create` could also fail, we need a second arm in the inner match
            // expression.
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                // When the file can't be create, a different error message is printed.
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // This arm of the match expression causes the program to panic on any error besides the
            // missing file error
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("{:?}", f);
}

// another way to write the above with closures
#[allow(dead_code)]
#[allow(unused_variables)]
fn closure_error_match() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);
}

// Using match works well enough, but it can be a bit verbose and and always communicate intent well
// The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those
// methods, called `unwrap`, is a shortcut method that is implemented just like the `match` expression.
// If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If
// the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.
#[allow(dead_code)]
#[allow(unused_variables)]
fn unwrap_example() {
    let f = File::open("hello.txt").unwrap();
}

// Another method, `expect`, which is similar to `unwrap`, lets us also choose the `panic!` error
// message. Using `expect` instead of `unwrap` and providing good error messages can convey your
// intent and make tracking down the source of a panic easier.
#[allow(dead_code)]
#[allow(unused_variables)]
fn expect_example() {
    // We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!`
    // macro. The error message used by `expect` in its call to `panic!` will be the parameter we
    // pass to `expect` rather than the default `panic!` messge that `unwrap` uses
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Propagating Errors
//
// When you're writing a function whose implementation calls something that might fail, instead of
// handling the error within this function, you can return the error to the calling code so that it
// can decide what to do. This is known as "propagating" the error and gives more control to the
// calling code, where there might be more information or logic that dictates how the error should
// be handled than what you have available in the context of the error causing code.
//
// Below shows a function that reads a username from a file. If the file doesn't exist or can't be
// read, this function will return those errors to the code that called the function.
use std::io;
use std::io::Read;

// This function is returning `Result<String, io::Error>`, this means the function is returning a
// value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the
// concrete type `String` and the generic type `E` has been filled in with the concrete type
// `io::Error`. If this function succeeds without any problems, the code that calls this function
// will receive an `Ok` value that holds a `String`--the username that this function read from the
// file. If this function encounters any problems, the code that calls this function will receive an
// `Err` value that holds an instance of `io::Error` that contains more information about what the
// problems were. We chose `io::Error` as the return type of this funcion because that happens to be
// the type of the error value returned from both of the operations we're calling in this function's
// body that might faile: the `File::open` function and the `read_to_string` method.
#[allow(dead_code)]
#[allow(unused_variables)]
fn read_username_from_file() -> Result<String, io::Error> {
    // The body of this function starts by calling the `File::open` function.
    let f = File::open("hello.text");
    // Then we handle the `Result` value returned with a `match` similar to how we did in the
    // `call_function_that_could_fail` funcion above
    let mut f = match f {
        // If `File::open` succeeds, we store the file handle in the variable `f` and continue
        Ok(file) => file,
        // only instead of calling `panic!` in the `Err` case, we return early from this function
        // and pass the error value from `File::open` back to the calling code as this function's
        // error value.
        Err(e) => return Err(e),
    };
    // Then we create a new `String` in variable `s`
    let mut s = String::new();
    // and call the `read_to_string` method on the file handle in `f` to read the contents of the
    // file into `s`. The `read_to_string` method also returns a `Result` because it might fail,
    // even though `File::open` succeeded. So we need another `match` to handle that `Result`:
    match f.read_to_string(&mut s) {
        // If `read_to_string` succeeds then our function has succeeded, and we return the username
        // from the file that now in `s` wrapped in an `Ok`.
        Ok(_) => Ok(s),
        // If `read_to_string` fails, we return the error value in the same way we returned the
        // error value in the `match` that handled the return value of `File::open`. However we
        // don't need to explicitly say `return`, because this is the last expression in the
        // function.
        Err(e) => Err(e),
    }
    // The code that calls this code will then handle getting either an `Ok` value that contains a
    // username or an `Err` value that contains an `io::Error`. We don't know what the calling code
    // will do with those values. If the calling code get an `Err` value, it could call `panic!` and
    // crash the program, use a default username, or look up the username from somewhere other than
    // a file, for example. We don't have enough information on what the calling code is actually
    // trying to do, so we propagate all the success or error information upward for it to be handled
    // appropriately.
    //
    // This pattern of propagating errors is so common in Rust that Rust provides the question mark
    // operator `?` to make this easier.
}

// Below shows an implementation of `read_username_from_file` that has the same functionality as it
// has above, but this implementation uses the `?` operator.
#[allow(dead_code)]
#[allow(unused_variables)]
fn read_username_from_file_2() -> Result<String, io::Error> {
    // The `?` placed after a `Result` value is defined to work in almost the same way as the `match`
    // expressions we defined to handle the `Result` values above. If the value of the `Result` is
    // an `Ok`, the value inside the `Ok` will get returned from this expression, and the program
    // will continue. If the value is an `Err`, the `Err` will be returned from the whole function
    // as if we had used the `return` keyword so the error value gets propagated to the calling code
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // There is a difference between the `match` expression used above and what the `?` operator
    // does: error values that have the `?` operator called on them go through the `from` function,
    // defined in the `From` trait in the standard library, which is used to convert error from one
    // type into another.
    //
    // When the `?` operator calls the `from` function, the error type received is converted into
    // the error type defined in the return type of the current function. This is useful when a
    // function returns one error type to represent all the ways a function might fail, even if
    // parts might fail for many different reasons. As long as each error type implements the `from`
    // function to define how to define how to convert itself to the returned error type, the `?`
    // operator takes care of the conversion automatically.
    //
    // In the context here, the `?` at the end of the `File::open` call will return the value inside
    // an `Ok` to the variable `f`. If an error occurs the `?` operator will return early out of the
    // whole function and five any `Err` value to the calling code. The same thing applies to the
    // `?` at the end of the `read_to_string` call.
    //
    // The `?` operator eliminates a lot of boilerplate and make this function's implementation
    // simpler. We could even shorten this code further like the code below
}

// Shorten the above examples by chaining method calls immediately after the `?`
#[allow(dead_code)]
#[allow(unused_variables)]
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    // We've moved the creation of the new `String` in `s` to the beginning of the function. Instead
    // of creating a variable `f`, we've chaine the call to `read_to_string` directly onto the
    // result of `File::open("hello.txt")?`. We still have `?` at the end of the `read_to_string`
    // call, and we still return an `Ok` value containing the username in `s` when both
    // `File::open` and `read_to_string` succeed rather than returning errors. The functionality is
    // again the same as above; just a different way to implement it.
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// We can make the above even shorter. Reading a file into a string is a fairly common operation, so
// Rust provides a convenient `fs::read_to_string` function that opens the file, creates a new
// `String`, reads the contents of the file, puts the contents into that `String, and returns it.
use std::fs;

#[allow(dead_code)]
#[allow(unused_variables)]
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The `?` Operator can be used in functions that return a result
//
// The `?` operator can be used in functions that have a return type of `Result`, it is defined to
// work in the same way as the `match` expression we defined earlier. The part of the `match` that
// requires a return type of `Result` is `return Err(e)`, so the return type of the function can be
// a `Result` to be compatible with this `return`.
//
// Let's look at what happens if we use the `?` operator in a function that has a return type of ()
#[allow(dead_code)]
#[allow(unused_variables)]
fn no_return_function() {
    // let f = File::open("hello.txt")?;
    // This won't compile we can only use the `?` operator in a function that returns `Result` or
    // `Option` or another type that implements `std::ops::Try`. When you're writing code in a
    // function that doesn't return one of these types, and you want to use `?` when you call other
    // functions that return `Result<T, E>`, you have two choices to fix this problem. One
    // technique is to change the return type of your function to be `Return<T, E>` if you have no
    // restrictions preventing that. The other technique is to use a `match` or one of the
    // `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.
}

// The `main` function is special, and there are restrictions on what its return type must be. One
// valid return type for main is (), and conveniently, another valid return type is `Result<T, E>, as
// shown below.
use std::error::Error;

// the `Box<dyn Error>` type is called a trait object, which means "any kind of error", in this
// context. Using `?` in a `main` function with this return type is allowed.
#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    // call_function_that_could_fail();
    // match_on_different_errors();
    // closure_error_match();
    let f = File::open("hello.txt")?;
    Ok(())
}
