// One detail we didn't discuss in the references_borrowing crate is that every reference in Rust
// has a "lifetime", which is the scope for which that reference is valid. Most of the time lifetimes
// are implicit and inferred, just like most of the time types are inferred. We must annotate types
// when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes
// of references could be related in a few different ways. Rust requires us to annotate the relationships
// using generic lifetime parameters to ensure the acutal references used at runtime will be definitely
// valid

// Preventing Dangling References with Lifetimes
//
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference
// data other than the data it's intended to reference. Consider this example, which has an outer
// scope and an inner scope
/*
fn example() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
*/
// Above won't compile because the value `r` is referring to has gone out of scope before we try to
// use it.

// Generic Lifetimes in Functions
//
// Let's write a function that returns the longer of two string slices. This function will take two
// string slices and return a string slice.
//
// This will not compile, the error help text reveals that the return type needs a generic lifetime
// parameter on it because Rust can't tell whether the reference being returned refers to `x` or `y`
// Actually, we dont' know either, because the `if` block in the body of this function returns a
// reference to `x` and the `else` block return a reference to `y`
/*
fn longest_bad(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    // When we're defining this function, we don't know the concrete values that will be passed into
    // this function, so we don't know whether the `if` case or the `else` case will execute. We also
    // don't know the concrete lifetimes of the references that will be passed in, so we can't look
    // at the scopes as we did before to determine whether the reference we return will always be
    // valid. The borrow checker can't determine this either, because it doesn't know how the lifetimes
    // of `x` and `y` relate to the lifetime of the return value. To fix this error, we'll add
    // generic lifetime parameters that define the relationship between the references so the borrow
    // checker can perform its analysis.
}
*/

// Lifetime Annotation Syntax
//
// Lifetime annotations don't change how long any reference lives. Just as functions can accept any
// type when the signature specifies a generic type parameter, functions can accept references with
// any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships
// of the lifetimes of multiple references to each other without effecting the lifetimes.
//
// Lifetimes annotations have a weird syntax: the names of a lifetime parameters must start with an
// apostrophe (') and are usually all lowercase and very short, like generic types. Most people use
// the name `'a`. We place lifetime parameter annotations after the `&` of a reference, using a space
// to separate the annotation from the reference's type.
/*

    let x: &i32; // a reference
    let x: &'a i32; // a shared reference with an explicit lifetime
    let x: &'a mut i32; // a mutable/exclusive reference with an explicit lifetime

*/
// One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to
// tell Rust how generic lifetime parameters of multiple references relate to each other. for example,
// let say we have a function with the parameter `first` that is a reference to an i32 with lifetime
// of `'a`. The function also has another parameter named `second` that is another reference to an
// i32 that also has a lifetime of `'a`. The lifetime annotations indicate that references `first` and
// `second` must both live as long as that generic lifetime.

// Lifetime Annotations in Function Signatures
//
// Now lets examine lifetime annotations in the context of the `longest` function. As with generic
// type parameters, we need to declare generic lifetime parameters inside angle brackets between the
// function name and the parameter list. The constraint we want to express in this signature is that
// all the references in the parameters and the return value must have the same lifetime.
//
// This function signature tells Rust that for some lifetime `'a`, the function takes two params both
// of which are string slices that live at least as long as lifetime `'a`. The return type is a string
// string slice will live at least as long as lifetime `'a`. In practice, it means that the lifetime
// of the reference returned by this function is the same a the smaller of the lifetimes of the
// references passed in. These constraints are what we want Rust to enforce. Remember, when we specify
// the lifetime parameters in this function signature, we're not changing the lifetimes of any values
// passed in or returned. Rather, we're specifying that the borrow checker should reject any values
// that don't adhere to these constraints.
//
// Notes that the `longest` function doesn't need to know exactly how lowng `x` and `y` will live,
// only that some scope can be substituted for `'a` that will satisfy this signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// When annotating lifetimes in functions, the annotations go in the function signature, not in the
// function body. Rust can analyze the code within the function without any help. However, when a
// function has references to or from code outside that function, it becomes almost impossbile for
// Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes
// might be different each time the function is called. This is why we need to annotaate the lifetimes
// manually.

// When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a`
// is part of the scope of `x` that overlaps with the scope `y`. In other words, the generic lifetime
// `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.
// Because we've annotated the returned reference with the same lifetime parameter `'a`, the returned
// reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

// Let's look at how the lifetime annotations restrict the `longest` function by passing in references
// that have different concrete lifetimes.
fn example_different_concrete_lifetimes_passed_to_longest() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Different Lifetimes: longest string is: {}", result);
    }
    // In this example, `string1` is valid until the end of the outer scope, `string2` is valid until
    // the end of the inner scope, and `result` references something that is valid until the end of
    // the inner scope. The borrow checker approves this code.
}

// Next, let's try an example that shows that the lifetime of the reference in `result` must be the
// the smaller lifetime of the two arguments.
/*
fn example_result_lifetime_is_smaller_of_two_params_lifetimes() {
    let string1 = String::from("long string is long");
    // We'll move the of the declaration of the `result` variable outside the inner scope but leave
    // the assignment of the value to the `result` variable inside the scope with `string2`
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // Then we'll move the `println!` that uses `result` outside the inner scoper, after the inner
    // scope has ended.
    println!(
        "Example Result Smallest Lifetime: longest string is: {}",
        result
    );
    // This does not compile. The error shows that for `result` to be valid for the `println!` statement
    // `string2` would need to be valid until the end of the outer scope. Rust knows this because we
    // annotated the lifetimes of the function parameters and return values using the same lifetime
    // parameter `'a`
    //
    // As humans, we can look at this code and see that `string1` is longer that `string2` and therefore
    // `result` will contain a reference to `string1`. Because `string1` has not gone out of scope
    // yet, a reference to `string1` will still be valid for the `println!` statement. However, the
    // compiler can't see that the reference is valid in this case. We've told Rust that the lifetime
    // of the reference return by the `longest` function is the same as the smaller of the lifetimes
    // of the references passed in. Therefore, the borrow checker disallows this code azs possibly
    // having an invalid reference.
}
*/

// Thinking in Terms of Lifetimes
//
// The way in which you need to specify lifetime parameters depends on what your function is doing. For
// example, if we changed the implementation of the `longest` function to always return the first
// parameterrather than the longest string slice, we wouldn't need to specify a lifetime on the `y`
// parameter
#[allow(unused_variables)]
#[allow(dead_code)]
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
    // Here we've specified a lifetime parameter `'a` for the parameter `x` and the return type, but
    // not for the parameter `y`, because the lifetime of `y` does not have nay relationship with the
    // lifetime of `x` or the return value.
}

// When returning a reference from a function, the lifetime parameter of the return type needs to match
// the lifetime parameter for one of the parameters. If the reference returned does not refer to one
// of the parameters, it must refer to a value created within this function, which would be a dangling
// reference because the value will go out of scope at the end of the function. Consider this attempted
// implementation of the `longest` function that won't compile:
/*
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
    // Here, even though we've specified a lifetime parameter `'a` for the return type, this implementation
    // will fail to compile because the return value lifetime is not related to the lifetime of the
    // parameters at all.
    //
    // The problem is that `result` goies out of sceope and get cleaned up at the end of this function.
    // We're also trying to return a reference to `result` from the function. There is no way we can
    // specify lifetime parameters that would change the dangling reference, and Rust won't let us
    // create a dangling reference. In this case, the best fix would be to return an owned data type
    // rather than a reference so the calling function is then responsible for cleaning up the value.
}
 */

// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values
// of functions. Once they're connected, Rust has enough information to allow memory safe operations
// and disallow operations that would create dangling pointers or otherwise violate memory safety.

// Lifetime Annotations in Struct Definitions
//
// So far, we've only defined structs to hold owned types. It's possible for structs to hold references,
// but in that case we would need to add a lifetime annotation on every reference in the struct's
// definition.
#[allow(dead_code)]
#[derive(Debug)]
// This struct has one field, `part`, that holds a string slice, which is a reference. As with generic
// data types, we declare the name of the generic lifefime parameter inside angle brackets after the
// name of the struct so we can use the lifetime parameter in the body of the struct definition. This
// annotation means an instance of `ImportantExcerpt` can't outlive the reference it holds in its
// `part` field.
struct ImportantExcert<'a> {
    part: &'a str,
}

// This function creates an instance of the `ImportantExcerpt` struct that holds a reference to the
// first sentence of the `String` owned by the variable `novel`. The data in `novel` exists before the
// `ImportantExcerpt` instance is created. In addition, `novel` doesn't go out of scope until after
// the `ImportantExcerpt` goes out of scope, so the reference in the struct instance is valid.
fn use_important_excerpt() {
    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcert {
        part: first_sentence,
    };
    println!("{:?}", i);
}

// Lifetime Annotations in Method Definitions
//
// When we implement methods on a struct with lifetimes, we use the same syntax as that of generic
// type parameters. Where we declare and us eht

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Equal Lifetimes: longest string is {}", result);
    example_different_concrete_lifetimes_passed_to_longest();
    // example_result_lifetime_is_smaller_of_two_params_lifetimes();
    use_important_excerpt();
}
