// Ownership Rules
//
// - Each value in Rust has a variable that's called it "owner"
// - There can only be one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {
    string_type();
    ways_variables_and_data_interact();
    clone_data();
    stack_only_data_copy();
    ownership_and_functions();
    return_values_scope();
    return_multiple_values_in_tuple();
}

// A scope is the range within a program for which an item is valid
fn _variable_scope() {
    // The variable `_s` refers to a string literal, where the value of the string is hard coded into
    // the text of the program. The variable is valid from the point at which it's declared until
    // the end of the current scope
    let _s = "hello"; // scope starts `_s` is valid from this point forward
                      // we do stuff with `_s`
} // this scope is now over, and `_s` is no longer valide

// All the variable types that we've used so far are all storeed on the stack and popped off the
// the stack when the scope is over
//
// String literals are immutable. They are of known size at compile time
// The `String` type is allocated on the heap and stores an unknown amount of text at compile time
fn string_type() {
    // Double colon is an operator that allows us to namespace this particular `from` function
    // under the `String` type rather than using some sort of name like `string_from`. This kind of
    // string can be mutated
    let mut s = String::from("Hello");
    // `push_str()` appends a literal to a String
    s.push_str(", world!");
    println!("{}", s);
    // Why can `String` be mutated by literals cannot? This difference is how these two types deal
    // with memory.
}

// With the `String` type, in order to support a mutable, growable piece of text, we to allocated an
// amount of memory on the heap, unknown at compile time, to hold the contents. That means...
//
// - the memory must be requested form the memory allocator at runtime
// - We need a way of returning this memory to the allocator when we're done with our `String`
//
// That first part is done by us: when we call `String::from`, its implementation requests the
// memory it needs. The second part is done by Rust automatically once the variable that owns it
// goes out of scope. Rust calls a special function called `drop` for us at the closing curly bracket

// Multiple variables can iteract with the same data in different ways in Rust.
fn ways_variables_and_data_interact() {
    // bind the value 5 to `x`
    let x = 5;
    // then make a copy of the value in `x` and bind it to `_y`, now have two variables that both
    // equal 5. Because integers are simple value of known, fixed size, now these two 5 values are
    // pushed onto the stack
    let _y = x;

    // this looks the same as about but it doesn't work the same.
    let s1 = String::from("hello");
    // When we assign `s1` to `s2`, the `String` type data is copied, meaning we copy the pointer,
    // the length, and the capacity that are on the stack. We do NOT copy the data on the heap that
    // the pointer refers to. Actually what happens is that this stack data is moved to `s2`
    let s2 = s1;
    // Below will not work because Rust considers `s1` to be invalid.
    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}

// If we do want deeply copy the heap data of the `String` type, not just the stack data we can use
// a common method called `clone`
fn clone_data() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// This code using integers - works and is valid. But this code seems to contradict what we just
// learned: we don't have to call `clone`, but `x` is still valid and wasn't moved to `y`
fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // the reason this works is that types such as integers that have a known size at compile time
    // are stored entirely on the stack, so copies of the actual values are quick to make. That means
    // there's no reason we would want to prevent `x` from being valid after we create the `y`
    // variable. In other words, there's no difference between deep and shallow copying here, so
    // calling `clone` wouldn't do anthing different from the usual shallow copying and we can
    // leave it out.
}

// Rust has a special annotation called `Copy` trait that we can place on types like integers that
// are stored on the stack. If a type has the `Copy` traint, an older variable is still usable after
// assignment.
//
// Rust won't let us annotate a type with the `Copy` trait if the type, or any of its parts, has
// implemented the `Drop` trait. If the type needs something special to happen when the value goes
// out of scope and we add the `Copy` annotation to that type, we'll get a compile time error.

// Here are some `Copy` types
// - All integer types, such as u32 or i32
// - The Boolean type `bool`, with values true and false
// - The character type `char`
// - Tuples, if they contain types that are also `Copy`, for example: `(i32, i32)` is `Copy`,
//      `(i32, String)` is not `Copy`

// The semantics for passing a value to a function are similar to those for assigning a value to a
// variable. Passing a variable to a function will move or copy, just as assignment does.
#[rustfmt::skip]
fn ownership_and_functions() {
    let s = String::from("Hello"); // `s` comes into scope

    takes_ownership(s); // `s`'s values into the function...
                                   // ... and so is no longer valid here
    let x = 5;                 // `x` comes into scope

    makes_copy(x);     // `x` would move into the function, but i32 is `Copy`, so it's
                                   // okay to still use `x` afterward
    println!("value of x: {}, is still valid", x);

} // Here, `x` goes out of scope, then `s`. But because `s`'s value was moved, nothing special
  // happens

#[rustfmt::skip]
fn takes_ownership(some_string: String) { // `some_string` comes into scope
    println!("{}", some_string);
} // Here, `some_string` goes out of scope and `drop` is called. The backing memory is freed

#[rustfmt::skip]
fn makes_copy(some_integer: i32) { // `some_integer` comes into scope
    println!("{}", some_integer);
} // Here, `some_integer` goes out of scope. Nothing special happens

// Returning values can also transfer ownership
#[rustfmt::skip]
fn return_values_scope() {
    let _s1 = gives_ownership(); // `gives_ownership` moves its return value into `s1`

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // `s2` is moved into `takes_and_gives_back`,
                                                      // which also moves its return value into `s3`
} // Here, s3 goes out of scope and is dropped. `s2` goes out of scope but was moved, so nothing
  // happens. `s1` goes out of scope and is dropped

#[rustfmt::skip]
fn gives_ownership() -> String {    // `gives_ownership` will move its return value into the function
                                    // that calls it
    
    let some_string = String::from("Hello"); // `some_string` comes into scope

    some_string // some string is returned and moves out to the calling function

}

// will take a `String` and return one
#[rustfmt::skip]
fn takes_and_gives_back(a_string: String) -> String { // `a_string` comes into scope

    a_string // `a_string` is returned and moves out to the calling function
}

// The ownership of a variable follows the same pattern every time: assigning a value to another
// variable moves it. When a variable that includes data on the heap goes out of scope, the value
// will be cleaned up by `drop` unless the data has been moved to be owned by another variable.
//
// Taking ownership and then return ownership with every function is a bit tedious. What if we
// want to let a function use a value but not take ownership? It's annoying that anything we pass in
// also needs to be passed back if we want to use it again, in addition to any data resulting from
// the body of the function that we might want to return as well.
//
// It's possible to return multiple values using a tuple
fn return_multiple_values_in_tuple() {
    let s1 = String::from("hello tuple");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // `len()` returns the length of a string
    (s, length)
}
