// Structs are similar to tuples. Like tupules, the pieces of a struct can be of diferent types.
// Unlike with tuples, you need to name each piece of data so it's clear what the values mean.
// As a result of these names, structs are more flexible that tuples: you don't have to rely on the
// order of the data to specify or access the values of an instance.

// To define a struct, we the keyword`struct` and name the entire struct. A struct's name should
// describe the significance of the pieces of data being grouped together. Then inside curly
// brackets we define the names and types of the pieces of data, which we call "fields".
// The struct definition is like a general template for the type, and instances fill in that template
// with particular data to create values of the type
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    instanciate_struct_use_value();
    instanciate_struct_change_field_value();
    println!(
        "Built User struct implicity returned: {:?}",
        build_user(String::from("some@email.com"), String::from("someusername"))
    );
    println!(
        "Built User struct field init shorthand: {:?}",
        field_init_shorthand(String::from("some@email.com"), String::from("someusername"))
    );
    struct_update_syntax();
}

fn instanciate_struct_use_value() {
    // To use a struct after we've defined it, we create an "instance" of that by specifying
    // concrete values for each of the fields. We create an instance by stating the name of the
    // struct and then add curly brackets containt `key: value` pairs, where the keys are the names
    // of the fields and the values are the data we want to store in those fields. Order of the
    // fields does not matter.
    let user1 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we use dot notations
    println!("User1 email address is: {}", &user1.email);
}

fn instanciate_struct_change_field_value() {
    // Note that the entire instance of the struct must be mutable; Rust doesn't allow us to mark
    // only certain fields as mutable.
    let mut user2 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User2 email before change: {}", &user2.email);
    user2.email = String::from("newemail@gmail.com");
    println!("User2 email after change: {}", &user2.email);
}

// As with any expression, we can construct a new instance of the struct as the last expression in
// the function body to implicity return that new instance
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn field_init_shorthand(email: String, username: String) -> User {
    // Here we are creating a new instance of the `User` struct, which has a field named "email".
    // We want to set the `email` field's value to the in the `email` parameter of this function.
    // Because the `email` field and the `email` parameter have the same name, we only need to
    // write `email` rather than `email: eamil,`
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_update_syntax() {
    let user1 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("user1name123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@gmail.com"),
        username: String::from("user2name123"),
        ..user1
    };
    println!("User1: {:?}", &user1);
    println!(
        "User2 created from User1 with struct update syntax: {:?}",
        &user2
    );
}

// You can also define structs that look similar to tuples called "tuple structs". Tuple strucks
// have the added meaning the struct name provides by don't have names associated with their fields;
// ranter, they just have the types of the fields. Tuple structs are useful when you want to give
// the whole tuple a name and make the tuple be a different type from other tuples, and naming
// each field like we do in a struct would be redundant or verbose.
//
// To define a tuple struct, start the `struct` keyword and th struct name followed by the types in
// in the tuple.
#[allow(dead_code)]
#[allow(unused_variables)]
fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // Note that the `black` and `origin` values are different types, because they're instances of
    // different tuple structs. Each struct you define is its own type, even though the fields
    // within the struct are the same types. For example, a function that takes a parameter of type
    // `Color` cannot take a `Point` as an argument, eventhough both types are made up of 3 i32
    // values. Otherwise, tuple struct instances behave like tuples: you can destructure them into
    // their individual pieces, you can use a (.) followed by the index to access an individual value
    // and so on.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit-Like Structs Witout any Fields
//
// Structs can be difined that don't have any fields...
// These are called "unit-like structs" because they behave similarly to `()`, the unit type.
//
// The `()` type has exactly one value `()`, and is used when there is no other meaningful value
// that could be returned. `()` is most commonly seen implicitly: functions without a `-> ...`
// implicityly have return type `()`, that is they are equivalent
//
// Unit-like structs can be useful in situations in which you need to implement a trait on some type
// but don't have any data that you want to store in the type itself.
#[allow(dead_code)]
struct SomeUnitStruct;

// In the `User` struct definition we used the owned `String` type rather than the `&str` string
// slice type. This was deliberate because we want instances of this struct to own all of its data
// and for that data to be valid for as long as the entire struct is valid.
//
// It's possible for structs to store references to data owned by something else, but to do so
// requires the use of "lifetimes". Lifetimes ensure that the data referenced by a struct is valid
// for as long as the struct is.
