// In the other enum crates we looked at how the `IpAddr` enum let us use Rust's type system to
// encode more information than just the data into our program. Here we will explore a case study
// of `Option`, which is another enum defined by the standard library.
//
// The `Option` type is used in many places because it encodees the very common scenario in which a
// value could be something or it could be nothing. Expressing this concept in terms of the type
// system means the compiler can check whether you've handled all the cases you be handling; this
// functionality can prevent bugs that extremely common in other programming lanugages.
//
// Programming language design is often though of in terms of which features you include, but the
// features you exclude are important too. Rust doesn't have the null feature that many other
// languages have. "Null" is a value that means that is no value there. In languages with null,
// variables can always be in one of two states: null or not-null.
//
// The problem with null values is that if you try to use a null value as a not-null value, you'll
// get an error of some kind. Because this null or not-null property is pervasive, it's extremely
// easy to make this kind of error.
//
// However, the concept that null is trying to express is still a useful one: a null is a value that
// is currently invalid or absent for some reason.
//
// The problem isn't really with the concept but with the particular implementation. As such, Rust
// does not have nulls, but it does have an enum that can encode the concept of a value being present
// or absent. This enum is `Option<T>` and is defined as follows
#[allow(dead_code)]
enum OptionEx<T> {
    Some(T),
    None,
}

// The `Option<T>` enum is so useful that it's even included in the prelude; you don't need to bring
// it into scope explicity. In addition, so are its variants `Some` and `None` directly without the
// `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are
// still variants of type `Option<T>`
//
// The `<T>` syntax is a generic type parameter. It means the `Some` variant of the `Option` enum can
// hold one piece of data of any type. Here are some examples of using `Option` values to hold
// number types and string types
#[allow(dead_code)]
#[allow(unused_variables)]
fn option_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // If we use `None` rather than `Some`, we need to tell Rust what type of `Option<T>` we have,
    // because the compiler can't infer the type that the `Some` variant will hold by looking only
    // at a `None` value.
    let absent_number: Option<i32> = None;
}

// When we have a `Some` value, we know that a value is present and the value is held with the `Some`.
// When we have a `None` value, in some sense, it means the same thing as null: we don't have a valid
// value. So why is have `Option<T>` any better than having null?
//
// In short, because `Option<T>` and `T` (where `T` can be any type) are different types, the
// compiler won't let us use an `Option<T>` value as if it were definitely a valid value.
#[allow(dead_code)]
#[allow(unused_variables)]
fn no_definite_use_of_option_t() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // The commented line below will not compile because it is trying to add an `i8` to an `Option<i8>`
    // We can't do this because they are different types. When we have a value of type like `i8` in
    // Rust, the compiler will ensure that we always have a valid value. We can proceed confidently
    // without having to check for null before using the value.
    //
    // In order to get below to compile you have to convert an `Option<T>` to a `T` before you can
    // perform `T` operations with it. Generally, this helps catch one of the most common issues
    // with null: assuming that something isn't null when it actually is.
    // let sum = x + y;
}

fn main() {
    println!("Hello, world!");
}
