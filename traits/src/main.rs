// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavior in an abstract way. We can use trait
// bounds to specify that a generic can be any type that has certain behavior

// Defining a Trait
//
// A Type's behavior consists of the methods we can call on that type. Different types share the same
// behavior if we can call the same methods on all those types. Trait definitions are a way to group
// method signatures together to define a set of behaviors necessary to accomplish some purpose.
//
// For example, let's say we have multiple structs that hold various kinds and amount of text: a
// `NewsArticle` struct that holds a news story filed in a particular location and a `Tweet` that
// can have at most 280 characters along with metadata that indicates whether it was a new tweet,
// a retweet, or a reply to another tweet.
//
// We want to make a media aggregator library that can disply summaries of data tha might be stored
// in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we need
// to request that summary by calling a `summarize` method on that instance. Below shows the
// definition of a `Summary` trait that expresses this behavior.

// Here, we declare a trait using the `trait` keyword and the trait's name, which is `Summary`
pub trait Summary {
    // Inside the curly brackets, we declare the method signatures that describe the behaviors of the
    // types that implement this trait.
    fn summarize(&self) -> String;
    // After the method signature, instead of providing an implementation within curly brackets, we
    // use a semicolon. The compiler will enforce that any type that has the `Summary` will have the
    // method `summarize` defined with this signature exactly
    //
    // A trait can have multiple methods in its body: the method signatures are list one per line
    // and each line ends with a semicolon
}

// Implementing a Trait on a Type
//
// Now that we've defined the desired behavior using the `Summary` trait, we can implement it on the
// types in our media aggregator.

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how we implement the Summary trait on the NewsArticle struct.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // The NewsArticle struct uses the headline, author and location to create the return value
        // of summarize
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // For the Tweet struct, we define summarize as the username followed by the entire text of the
    // tweet, assuming that tweet content is already limited to 280 characters.
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implementing a trait on a type is similar to implementing regular methods. The difference is that
// after `impl`, we put the trait name that we want to implement, then use the `for` keyword, and
// then specify the name of the type we want to implement the trait for. Within the `impl` block, we
// put the method signatures that the trait definition has defined. Instead of adding a semicolon
// after each signature, we use curly brackets and fill in the method body with the specific behavior
// that we want the methods of the trait to have for the particular type

// Default Implementations
//
// Sometimes it's useful to have default behavior for some or all of the methods in a trait instead
// of requiring implementations for all methods on every type. Then, as we implement the trait on a
// particular type, we can keep or override each method's default behavior.

// Below shows how to specify a default string for the `summarize` method of the `Summary2` trait
// instead only defining the method signature
pub trait Summary2 {
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

// To use a default implementation to summarize instances of `NewsArticle` instead of defining a
// custom implementation, we specify an empty `impl` block with `impl Summary2 for NewsArticle{}`
impl Summary2 for NewsArticle {}

// Default implementations can call other methods in the same trait, even if those other methods
// don't have a default implementation. In this way, a trait can provide a lot of useful functionality
// and only require implementors to specify a small part of it. For example, we could define the
// `Summary` trait to have a `summarize_author` method whose implementation is required, and then
// defines a `summarize` method that has default implementation that calls the `summarize_author`
// method.

pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// to use the above version of `Summary3` we only need to define `summarize_author` when we implement
// the trait on a type
impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
//
// Now that you know to define and implement traits, we can explore how use traits to define functions
// that accept many different types.
//
// For example, above, we implemented `Summary` trait on the `NewsArticle` and `Tweet` types. We can
// define a `notify` function that calls the `summarize` method on its `item` parameter, which is
// of some type that implements the `Summary` trait.
pub fn notify(item: &impl Summary) {
    // Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the
    // trait name. This parameter accepts any type that implement the specified trait. In the body
    // of `notify`, we can call any methods on `item` that come from `Summary` trait, such as
    // `summarize`. We can call `notify` and pass any instance of `NewsArticle` or `Tweet`. Code
    // that calls this function with any other type, such as `String` or an i32, won't compile
    // because those types don't implement `Summary`
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
//
// The `impl Trait` syntax works for straightforward cases bit it's actually syntax sugar for a longer
// form, which is called a "trait bound", it looks like this
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    // This longer form is equivalent to the other example but is more verbose. We place trait bounds
    // with the declaration of the generic type parameter after a colon and inside angle brackets.
}

// The `impl Trait` syntax is convenient and make for more concise code in simple casrs. The trait
// bound syntax can express more complexity in other cases. For example, we can have two parameters
// that implement `Summary`. Using the `impl Trait` syntax looks like this...
#[allow(unused_variables)]
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    // code here
}

// If we wanted the above function to allow `item1` and `item2` to have different types, using impl
// Trait syntax would be appropriate (as long as both types implement `Summary`). If wanted to force
// both parameters to thave the same type, that's only possible to express using a trait bound like this...
#[allow(unused_variables)]
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    // code here
    // The generic type `T` specified as the type of the `item1` and `item2` parameters constrains
    // the function such that the concrete type of the value passed as an argument for `item1` and
    // `item2` must be the same
}

// Specifying Multiple Trait Bounds with the + Syntax
//
// We can also specify more than one trait bound. Say we wanted `notify` to use display formatting on
// `item` as well as the `summarize` method: we specify in the `notify` definiton that `item` must
// implement both `Display` and `Summary` like this
use std::fmt::{Debug, Display};

#[allow(unused_variables)]
pub fn notify5(item: &(impl Summary + Display)) {
    // code here
}

// The + syntax is also valid with trait bounds on generic types. With the two trait bounds specified
// the body bofy of `notify` can call `summarize` and use `{}` to format `item`
#[allow(unused_variables)]
pub fn notify6<T: Summary + Display>(item: &T) {
    // code here
}

// Clearer Trait Bounds with `where` Clauses
//
// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions
// with multiple generic type parameters can contain lots of trait bound infomration between the
// function's name and its parameter list, making the function signature hard to read. For this
// reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the
// function signatures, so instead of writing this...
#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // code here
    3
}

// we can use a `where` clause, like this:
#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Code here
    3
    // This function's signature is less cluttered: the function name, parameter list, and return
    // type are close together, similar to a function without lots of trait bounds
}

// Returning Types that Implement Traits
//
// We can also use the `impl Trait` syntax in the return position to return a value of some type that
// implements a trait, as shown here
#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    // By using `impl Summary` for the return type, we specifgy that this function returns some type
    // that implements the `Summary` trait without naming the concrete type. In this case, this
    // function returns a `Tweet`, but the code calling the function doesn't know that
    //
    // The ability to return a type that is only specified by the trait it implements is especially
    // useful in the context of closures and iterators. Closures and iterators create types that only
    // the compiler knowns or types that very long to specify. The `impl Trait` syntax let you
    // concisely specify that a function return some type that implements the `Iterator` trait without
    // needing to write out a very long type.
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// You can only use `impl Trait` if you're returning a single type. For example, the code below that
// returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary`
// wouldn't work
/*
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

// Fixing the `largest` function with Trait Bounds
//
// Now that we know how to specify the behavior you wan to use using the generic type parameter's
// bounds, let's return to the crate 'generic_data_types' and fix the definition of the `largest`
// function that uses a generic type parameter. Last time we tried to run that code, we received
// this error...
/*
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
  = note: `T` might need a bound for `std::cmp::PartialOrd`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
*/
// In the body of `largest` we wanted to compare two values of type `T` using the the greater than
// operator. Because the operator is defined as a default method on the standard library trait
// `std::cmp::PartialOrd`, we need to specify `PartialOrd` in the trait bounds for `T` so the
// `largest` function can work on slices of any type that we can compare. We don't need bring
// `PartialOrd` into scope because it's in the prelude. The new signature of `largest` looks like
// this...
/*
fn largest<T: PartialOrd>(list: &[T]) -> T {
    // code here

}*/
// This time when we compile the code, we get a different set of errors
/*
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
*/
// The key line in this error is `cannot move out of [T], a non-copy slice`. With out non-generic
// versions of the `largest` function, we were only trying to find the largest i32 or char. As
// discussed, types like i32 and char that have a known size can be stored on the stack, so they
// implement the `Copy` trait. But when we made the `largest` function generic, it became possible
// for the `list` parameter to have types in it that don't implement the `Copy` trait. Consequently,
// we wouldn't be able to move the out of `list[0]` and into the `largest` variable, resulting in
// the above error.
//
// To call this code with onlyu those types that implemnt the `Copy` trait, we can add `Copy` to the
// traint bounds of `T`. Below shows the complete code of a generic `largest` function that will
// compile as long as the types of the values in the slice that we pass into the function implement
// the `PartialOrd` and `Copy` traits, like i32 and char do.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
    // if we don't want to restrict this function to the types that implement the `Copy` trait, we
    // could specify that `T` has the trait bound `Clone` instead of `Copy`. Then we couuld clone
    // each value in the slice when we want this function to have ownership. Using the `clone`
    // function means we're potentially making more heap allocations in the case of types that own
    // heap data like `String`, and heap allocations can be slow if we're working with large amounts
    // data.
    //
    // Another way we could implement this function is for the function to return a reference to a
    // `T` value in the slice. If we change the return type to `&T`, thereby changing the body of the
    // function to return a reference, we wouldn't need the `Clone` or `Copy` trait bounds and could
    // avoid heap allocations.
}

// Using Trait Bounds to Conditionally Implement Methods
//
// Buy using a trait bound with an `impl` block that uses generic type parameters, we can implement
// methods conditionally for types that implement the specified traits. For example, we the type
// `Pair<T>` always implements the `new` function. But `Pair<T>` only implements the `cmp_display`
// method if its inner type `T` implements the `PartialOrd` trait that enables comparison and the
// `Display` trait that enables printing.
#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };
    println!("News article available! {}", article.summarize_default());

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet2.summarize3());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
