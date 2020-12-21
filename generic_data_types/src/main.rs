// Generic Data Types
//
// We can use generics to create definitions for items like function signatures or structs, which
// we then use with many different concrete data types. Let's first look at how to define functions,
// structs, enums, and methods using generics. Then we'll discuss how generics affect code performance

// In Function Definitions
//
// When defining a function that uses generics, we place the generics in the signature of the function
// where we would usually specify the data types of the parameters and return value. Doing so makes
// our code more flexible and provides more functionality to callers of our function while prventing
// code duplication.
//
// Continuing with our `largest` function, that introduced in the generic_types_trairs_lifetimes_intro
// crate, below shows two functions both find the largest value in a slice.

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The `largest_i32` bunction is the one we extracted in the generic_types_trairs_lifetimes_intro
// crate, that finds the largest i32 in a slice. The `largest_char` function find the larget char
// in a slice. The function bodies have the same code, so let's eliminate the duplication by
// introducting a generic type parameter in a single function.
//
// To parameterize the types in the new function we'll define, we need to name the type parameter,
// just as we do for the value parameter to a function. You can use any identifier as a type
// paramter name. But we'll use `T` because, by convention, paramter names in Rust are short, often
// just a letter, and Rust's type-naming convention is CamelCase. Short for "type", `T` is the
// default choice of most Rust programmers.
//
// When we use a parameter in the body of the function, we have to declare the parameter name in the
// signature so the compiler knows what that name means. Similarly, when we use a type paramter name
// in a function signature, we have to declare the type parameter name before we use it. To define
// the generic `largest` function, place type name declaration inside angle brackets, `<>`, between
// the name of the function and the parameter list, like below
//
// We read this definition as: the function `largest` is generic over some type `T`. This function
// has one parameter nameed `list`, which is a slice of values of type `T`. The `largest` function
// will return a reference to a value of the same type `T`.
/*fn largets<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // below will not compile, the error message states: "note: `T` might need a bound for
        // `std::cmp::PartialOrd`". The note mentions `std::cpm::PartialOrd`, which is a trait. We
        // will cover traits in other crates. For now, this error states that the body of `largest`
        // won't work for all possible types that `T` could be. Because we to compare values of type
        // `T` in the body, we can only use types whose values can be ordered. To enable comparisons
        // the standard library has the `std::cmp::PartialOrd` trait that you can implement on types
        if item > largest {
            largest = item;
        }
    }

    largest
} */

// In Struct Definitions
//
// We can also define structs to use a generic type paramter in one or more fields using the `<>`
// syntax. Below shows how to define a `Point<T>` struct to hold `x` and `y` coordinate values of
// any type
//
// The syntax for using generic in struct definitions is similar to that used in function definitions.
// First, we declare the name of the type parameter inside angle brackets just after the name of the
// struct
#[allow(dead_code)]
struct Point<T> {
    // Then we can use the generic type in the struct definition where wouold otherwise specify
    // concrete data types
    x: T,
    y: T,
}
/*
fn example_bad_instantiation() {
    // Types have to be the same, here we use i32 for `x` and f32 for `y`
    let wont_work = Point { x: 5, y: 4.0 };
}
*/

// To define a `Point` struct where `x` and `y` are both generics but could have different types, we
// can use multiple generic type parameters. For example, below we can change the definition to be
// generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`
#[allow(dead_code)]
struct PointMultipleTypeParameters<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn instantiate() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = PointMultipleTypeParameters { x: 5, y: 4.0 };
}

// In Enum Definitions
//
// As we did with structs, we can define enums to hold generic data types in their variants.
//
// This definitions should now make more sense. As your can see `Option<T>` is an enum that is
// generic over type `T` and has two variants: `Some`, which holds one value of type `T`, and a
// `None` variant that doens't hold any value. By using the `Option<T>` enum, we can express the
// abstract concept of having an option value, and because `Option<T>` is generic we can use this
// abstraction no matter what type of the optional value is.
#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

// Enums can use multiple generic types as well. The definition of the `Result` enum that we used
// in previous crates is one example
#[allow(dead_code)]
enum Result<T, E> {
    // The `Result` enum is generic over two types, `T` and `E`, and has two variants: `Ok`, which
    // hold a value of type `T`, and `Err`, which holds a value of type `E`. This definition makes
    // it convenient to the `Result` enum anywhere we have an operation that might succeed (return a
    // a value of some type `T`) or fail (return an error of some type `E`)
    Ok(T),
    Err(E),
}

// In Method Definitions
//
// We can implement methods on structs and enums and use generic type in their definitions as well.

// Below we show the `Point<T>` struct with a method named `x` implemented on it
// Note that we have to declare `T` just after `impl` so we can use it to specify that we're
// implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`,
// Rust can identify that the type in the angle brackets in `Point` is a generic type rather than
// a concrete type,
impl<T> Point<T> {
    // Here we've defined a method named `x` on `Point<T>` that returns a reference to the data in
    // the field `x`
    fn x(&self) -> &T {
        &self.x
    }
}

// We could, for example, implement methods only on `Point<f32>` instances rather than on `Point<T>`
// instances with any generic type.
impl Point<f32> {
    // This code meands the type of `Point<f32>` will have a method name `distance_from_origin` and
    // other instance of `Point<T>` where `T` is not of type f32 will not have this method defined.
    // The method measures how far a point is from the point at coordinates (0.0, 0.0) and uses
    // mathematical operations that are available only for floating point types
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic type parameters in a struct definition aren't alway the same as those you use in that
// struct's mehtod signatures. For example, below defines the method `mixup` and the `PointMultipleTypeParameters<T, U>`
// struct.

impl<T, U> PointMultipleTypeParameters<T, U> {
    // This method takes another `PointMutlitpleTypeParameters` as a paramter, which migh have
    // different types form the `self` `PointMultipleTypeParameters` we're calling `mixup` on. This
    // method create a new `PointMutlipleTypeParameters` instance with the `x` value from the `self`
    // `PointMultipleTypeParameters` (of type `T`) and the `y` value from the passed-in
    // `PointMultipleTypeParameters` (of type `W`)
    fn mixup<V, W>(
        self,
        other: PointMultipleTypeParameters<V, W>,
    ) -> PointMultipleTypeParameters<T, W> {
        PointMultipleTypeParameters {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2: Point<f32> = Point { x: 5.0, y: 10.0 };
    println!("Distance from Origin is: {}", p2.distance_from_origin());
    // Here we've defined a `PointMutlipleTypeParameters` that has an i32 for `x` (with value of 5)
    // and an f64 for `y` (with a value of 10.4). The `p4` variable has a string slice for `x` (with
    // a value of "Hello") and a char for `y` (with a value of 'c').
    let p3 = PointMultipleTypeParameters { x: 5, y: 10.4 };
    let p4 = PointMultipleTypeParameters { x: "Hello", y: 'c' };
    // Calliing `mixup` on `p3` with the argument `p4` gives us `p5`, which will have an i32 for `x`
    // because `x` came from `p3`. The `p5` variable will have a char for `y`, because `y` came from
    // `p4`
    let p5 = p3.mixup(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}

// Performance of Code Using Generics
//
// You might be wondering whether there is a runtime cost when you're using generic type parameters.
// The good news is that Rust implements generics in such a way that your code doens't run any slower
// using generic types than it would with concrete types.
//
// Rust accomplishes this by performing monomorphization of the code that is using generics at compile
// time. "Monomorphization" is the process of turning generic code into specific code by filling in
// the concrete types that used when compiled.
