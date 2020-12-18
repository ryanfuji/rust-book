// This enum has 4 variants
// - `Quit` has no data associated with it at all
// - `Move` includes an anonymous struct inside it
// - `Write` includes a single `String`
// - `ChangeColor` includes three i32 values
#[allow(dead_code)]
enum Message {
    Quite,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Defining an enum with variant like above is similar to defining different kinds of struct
// definitions, except that the enum doesn't use the `struct` keyword and all the variants are grouped
// together under the `Message` type. These structs could hold the same data that the above enum holds
#[allow(dead_code)]
struct QuitMessage; // unit struct
#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
struct WriteMessage(String); // tuple struct
#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, which each have their own type, we couldn't easily define
// a function to take any of these kinds of messages as we could with the `Message` enum defined
// above
//
// There is one more similarity between structs and enums: just as we're able to define methods structs
// using `impl`, we're also able to define methods on enums. Here's a method named `call` that we
// could define on our `Message` enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // The body of the method implemented on the `Message` enum would use `self` to get the value
    // that we called the method on. Below we've created a variable `m` that has the value
    // `Message::Write(String::from("hello"))`, and that is what `self` will be in the body of the
    // `call` method when `m.call()` runs
    let m = Message::Write(String::from("hello"));
    m.call();
}
