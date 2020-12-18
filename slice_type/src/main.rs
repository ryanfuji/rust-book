fn main() {
    first_string_slice();
    slice_start_at_beginning();
    slice_include_last_byte();
    slice_drop_start_and_end_index();
    call_first_word();
    call_string_sices_as_parameters();
    other_slices();
}

// Another data type that not have ownership is the slice. Slices let you reference a contiguous
// sequence of elements in a collection rather than the whole collection

// Lets try and write a function that takes a string and returns the first word it finds in that
// string. If the function doesn't find a space in the string, the whole string must be one word,
// so the entire string should be returned

// This function, `first_word`, has a `&String` as a parameter. We don't want ownership, so this is
// fine. But what should we return? We don't really have a way to talk about part of a string.
// However, we could return the index of the end of the word.
#[allow(dead_code)]
fn first_word(s: &String) -> usize {
    // Because we need to go through the `String` element by element and check whether a value is a
    // space, we'll convert our `String` to an array of bytes using the `as_bytes()` method
    let bytes = s.as_bytes();

    // Next, we create an iterator over the array of bytes using the `iter()` method
    // an `iter` is a method that returns each element in a collection and `enumerate` wraps the
    // result of `iter` and returns each element as part of a tuple instead. The first element of the
    // tuple returned from `enumerate` is the index, and the second element is a reference to the
    // element.
    // Because `enumerate` method returns a tuple, we can use patterns to destructure that
    // tuple, just like everywhere else in Rust. So in the `for` loop, we specify a pattern that
    // has `i` for the index and `&item` for the single byte in the tuple. Because we get a reference
    // from `iter().enumerate()`, we use `&` in the pattern
    for (i, &item) in bytes.iter().enumerate() {
        // We search for the byte that represents the space by using the byte literal syntax, `b' '`
        // If we find a space, we return the position. Otherwise we return the length of the string
        // by using `s.len()`
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn call_first_word() {
    let mut s = String::from("hello world");
    // word will get the the value 5
    let word = first_word_rewrite(&s);

    println!("First Word: {}", word);

    // this empties the string, making it equal to ""
    s.clear();

    // word still have the value 5 here, but there's no more string that we meaningfully use the
    // value 5 with. word is now invalid

    // This program compiles without any errors and also do so if we used `word` after calling
    // `s.clear()`. Because `word` isn't connected to the state of `s` at all, `word` still contains
    // the value 5. We could use that value 5 with the variable `s` to try to extract the first
    // word out, but this would be a bug because the contents of `s` have changed since we saved 5
    // in `word`
}

// String Slices to rescue
// A string slice is a reference to part of a `String`, and it looks like this
// This is similar to taking a reference to the whole `String` but with the extra `[0..5]` bit.
#[allow(dead_code)]
fn first_string_slice() {
    let s = String::from("hello world");
    // create slice using a range within brackets by specifying
    // [starting_index..ending_index]
    // where starting_index is the first position within the slice and ending_index is one more than
    // the last position in the slice
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("String Slice: {} {}", hello, world);
}

// Internally, the slice data structure stores the starting position and the length of the slice,
// which corresponds to ending_index minus starting_index. So in the of case of
// `let world = &s[6..11];`, `world` would be a slice that contains a pointer to the 7th byte
// (counting from 1) of `s` with a length value of 5

// With Rust's `..` syntax, if you want to start at the first index (zero), you can drop the value
// before the two periods the following are equivalent
fn slice_start_at_beginning() {
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("With Zero: {}", slice);
    let slice = &s[..2];
    println!("Without Zero: {}", slice);
}

// Also if your slice includes the last byte of the `String`, you can drop the trailing number
fn slice_include_last_byte() {
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("With trailing: {}", slice);
    let slice = &s[3..];
    println!("Without trailing: {}", slice);
}

// Also can drop both start and end to take a slice of the entire string
fn slice_drop_start_and_end_index() {
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("With Indexes: {}", slice);
    let slice = &s[..];
    println!("Without Indexes: {}", slice);
}

// Note: String slice range indexes must occur at valid UTF-8 character boundaries. If you attempt
// to create a string slice in the middle of a multibyte character, you will get an error. For
// the purposes of introducing string slices, we are assuming ASCII only here.

// The type that denotes a "string slice" is written as `&str`
fn first_word_rewrite(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String literals are stored inside the binary, now that we know about slices we can better
// understand string literals
#[allow(dead_code)]
#[allow(unused_variables)]
fn string_literals_are_slices() {
    // The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; `&str` is an immutable/shared reference
    let s = "Hello, world!";
}

// Knowing that you can take clices of literals and `String` values leads us to one more improvement
// on `first_word`, and that's its signature `fn first_word(s: &String) -> &str {`
// We could write this signature instead because it allows us to use the same function on both
// `&String` and `&str` values
// `fn first_word(s: &str) -> &str {`
fn call_string_sices_as_parameters() {
    let my_string = String::from("hello world");

    // `first_word_rewrite2()` works on slices of `String`s
    let word = first_word_rewrite2(&my_string[..]);
    println!("First Word from slice of String type: {}", word);

    let my_string_literal = "literal hello world";

    // `first_word_rewrite2` works on slices of string literals
    let word = first_word_rewrite2(&my_string_literal[..]);
    println!("First Word from slice of string literal: {}", word);

    // Because string literals *are* string slices already, this works too, without the slice
    // syntax
    let word = first_word_rewrite2(my_string_literal);
    println!(
        "First Word from slice of string literal, no slice syntax: {}",
        word
    );
}

fn first_word_rewrite2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    // To refer to part of an array do this. This slice has a type of `&[i32]`. It works the same
    // way as string slices do, by storing a reference to the first element and a length.
    let slice = &a[1..3];
    println!("Slice of Array: {:?}", slice);
}
