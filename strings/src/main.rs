// String are implemented as a collection of bytes, plus some methods to provide useful functionality
// when those bytes are interpreted as tex.
//
// Rust has only one string type in the core language, which is the string slice `str` that is
// usually seen in its borrowed form `&str`.
//
// The `String` type, which is in Rust's standard library not in the core language, is a growable,
// mutable, owned, UTF-8 encoded string type. When Rustaceans refer to "strings" in Rust, they
// usually mean the `String` and the string slice `&str` types, not just one of those types. Both
// types are heavily used in Rust's standard library, and both `String` and string slices are
// UTF-8 encoded
//
// Rust's standard library also includes a number of other string types, such as `OsString`, `OsStr`,
// `CString`, and `CStr`. Library crates can provide even more options for storing string data. See
// how all those names all end in `String` or `Str`? They refer to owned and borrowed variants, just
// like the `String` and `str` types we've been working with. These string types can store text in
// different encodings or be represented in memory in a different way, for example.

// Many of the same operations available with `Vec<T>` are available with `String` as well, starting
// with the `new` function to create a string
#[allow(unused_mut)]
fn create_string_with_new() {
    // This line creates a new empty strting `s`, which we can then load data into
    let mut s = String::new();
    println!("Empty string with new function: {:?}", &s);
}

// Often, we'll have some initial data that we want to start the string with. For that, we use the
// `to_string` method, which is available on any type that implements the `Display` trait, as string
// literals do
fn init_string_with_to_string() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", &s);
    // this method also work on a literal directly
    let s = "initial contents direct".to_string();
    println!("{}", &s);
    // We can also use the function `String::from` to create a `String` from a string literal. This code
    // is equivalent to `to_string()`
    let s = String::from("initial contents from String");
    println!("{}", &s);
}

// A `String` can grow in size and it contents can change just like the contentx of a `Vect<T>`, if
// you push more data into it. You can also use the `+` operator or the `format!` macro to
// concatenate `String` values.
//
// We grow a `String` by using the `push_str` method to append a string slice
fn use_push_str_append_str_slice() {
    let mut s = String::from("foo");
    // Using `&mut s` here so we don't get compiler error about multiple borrowing types in same
    // scope
    println!("{}", &mut s);
    // this method takes a string slice because we don't necessarily want to take ownership of the
    // parameter
    s.push_str(" bar");
    println!("{}", &s);
}

// Here the `push_str` method takes ownership of `s2`,
fn push_str_take_ownership() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", &s2);
    println!("s1 is {}", &s1);
}

// Often you'll want to combine two existing strings. One way is to use the `+` operator
#[rustfmt::skip]
fn cat_strings_with_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!!");
    let s3 = s1 + &s2; // NOTE `s1` have been moved here and can no longer
                                                   // be used
    println!("{}", &s3);
    // The reason `s1` is no longer valid after the addition and the reason we used a reference to
    // `s2` has to do with the signature of the method that gets called when we use the `+` operator
    // The `+` operator uses the `add` method, whose signature looks something like this...
    //      fn add(self, s: &str) -> String
    // This isn't the exact signature that's in the standard library: in the standard library, `add`
    // is defined using generics. Here, we're looking at the signature of `add` with the concrete
    // types substituted for the generic ones, which is what happens when we call this method with
    // `String` values.
    // 
    // First, `s2` has an `&`, meaning that we're adding a reference of the second string to the first
    // string because of the `s` parameter in the `add` function: we can only add a `&str` to a 
    // `String`; we can't add two `String` values together. But wait--the type of `&s2` is a `&String`,
    // not a `&str`, as specified in the second parameter to `add`. So what's going on?
    //
    // The reason we're able to use `&s2` in the call to `add` is that the compiler can 'coerce' the
    // `&String` argument into a `&str`. When we call the `add` method, Rust uses a "deref coercion",
    // which here turns `&s2` into `&s2[..]`. Because `add` does NOT take ownership of the `s`
    // parameter, `s2` will still be a valid `String` after this operation.
    //
    // Second, we can see in the signature that add takes ownership of `self`, because `self` does
    // NOT have an `&`. This means `s1` will be moved into the `add` call and no longer valid after 
    // that. So although `let s3 = s1 + &s2;` looks like it will copy both string and create a new
    // one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`
    // and then return ownership of the result. In other words, it looks like it's making a lot of
    // copies but isn't; the implementation is more efficient than copying.
}

// if we need to cat multiple string, the behavior of the `+` operator gets unwieldly:
fn cat_mult_strings_plus() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}

// It is a better idea to do the same as above with the `format!` macro instead
fn cat_mult_strings_println_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", &s1, &s2, &s3);
    println!("{}", s);
}

fn main() {
    create_string_with_new();
    init_string_with_to_string();
    use_push_str_append_str_slice();
    push_str_take_ownership();
    cat_strings_with_plus();
    cat_mult_strings_plus();
    cat_mult_strings_println_macro();
}
