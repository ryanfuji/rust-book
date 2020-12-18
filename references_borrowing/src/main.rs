fn main() {
    no_ownership_required();
    try_modify_reference();
    modify_mutable_reference();
    try_multiple_exclusive_refs();
    multiple_non_simultaneous_exclusive_references();
    try_to_combine_references();
    reference_mixing_scope_caveat();
}

// The issue with the tuple code in the what_is_ownership crate is that we have to return the
// `String` to the calling function so we can still use the `String` after the call to the
// `calculate_length` function, because the `String` was moved into the function.

// Here is how you would define and use a `calculate_length` function that has a reference to an
// object as a parameter instead of taking ownership of the value
fn no_ownership_required() {
    let s1 = String::from("hello");
    // the `&s1` lets us create reference that refers to the value of `s1` but does not transfer
    // ownership
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // because ownership is not transferred the value is not dropped after here
    s.len()
}

fn try_modify_reference() {
    let s = String::from("Hello");
    change(&s);
}

fn change(_some_string: &String) {
    // below will not work because trying to mutate a borrowed reference
    // some_string.push_str(", world");
}

fn modify_mutable_reference() {
    // first have to make `s` mutable
    let mut s = String::from("Hello");
    // pass exclusive reference to `change_mutable`
    change_mutable(&mut s);
    // `s` is still valid because we didn't transfer ownership
    println!("{}", s);
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world!");
}

// Mutable/Exclusive references have one big restriction: you can only have one to a piece of data
// at any one time
fn try_multiple_exclusive_refs() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // compiler will not allow mutiple exclusive refs to `s`
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("r1 is Exclusive reference to s: {}", r1);
}

// The benefit of having the single exclusive reference rule is that Rust can prevent data races
// at compile time. A data race is similar to a race condition and happens when these 3 things
// occur
// - Two or more pointers access the same data at the same time
// - At least one of the pointers is being used to write the data
// - There's no mechanism being used to synchronize access to the data

// We can use curly brackets to create a new scope, allowing for multiple exclusive references
// but just not simultaneous ones
fn multiple_non_simultaneous_exclusive_references() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 exclusive reference to s: {}", r1);
    } // `r1` goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;
    println!("r2 exclusive reference to s: {}", r2);
}

// A similar rules exists for combining mutable/exclusive and immutable/shared references
#[rustfmt::skip]
#[allow(unused_mut)]
fn try_to_combine_references() {
    let mut s = String::from("Hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // below will not work can't combine exclusive and shared refs to same value
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("r1 is ok: {}, r2 is ok: {}", r1, r2);
}

// We also cannot have a mutable/exclusive reference while we have an immutable/shared one. Users of
// an immutable/shared reference don't expect the values to suddenly change out from under them.
// However multiple immutable/shared references are okay because no one who is just reading the data
// has the ability to affect anyone else's reading of the data
//
// Note that a reference's scope starts where it is introduced and continues through the last time
// that reference is used. For instance, this code will compile because the last usage of the
// immutable/shared references occurs before the mutable/exclusive reference is introduced.
//
// The scopes of the immutable/shared references `r1` and `r2` end after the `println!` where they
// are last used, which is before the mutable/exclusive reference `r3` is created. These scopes
// don't overlap, so this code is allowed
#[rustfmt::skip]
fn reference_mixing_scope_caveat() {
    let mut s = String::from("YoYoYo");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1 is good: {}, r2 is good: {}", r1, r2);
    // `r1` and `r2` are no longer used after this point

    let r3 = &mut s; // no problem
    println!("r3 is also good: {}", r3);
}

// No dangling pointers possible,  a pointer that references a location in memory that may have
// been given to someone else, by freeing some memory while preserving a pointer to that memory.
// The two function commented out below will not compile because they are trying to create a
// dangling pointer
// fn try_to_create_dangling_pointer() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
