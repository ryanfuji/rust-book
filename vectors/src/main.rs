// The first collection type we'll look at is the `Vec<T>`, also known as a "vector". Vectors
// all you to store more than value in a single data structure that puts all values next to each
// other in memory. Vectors can only store values of the same type They are useful when you have a
// list of items, such as the of lines of text in a fie or the prices of items in a shopping cart.

// To create a new, empty vector we can call the `Vec::new` funcion
#[allow(unused_mut)]
fn creat_empty_vector() {
    // Note that we added a type annotation, when we aren't inserting any values into this vector,
    // Rust doesn't know what type the items in the vector will be with it
    let v: Vec<i32> = Vec::new();
    println!("Empty vector is: {:?}", &v);
    // In this vector Rust can infer the type from the values we inserted
    let mut v2 = vec![1, 2, 3];
    println!("Infered type vector is: {:?}", &v2);
    // Like any other struct, a vector is freed whne it goes out of scope. When the vector gets dropped
    // all of its contents are also, dropped meaning those integers it holds will be cleaned up.
} // <- both `v` and `v2` goes out of scope and is freed here

// To create a vector and then add elements to it, we can use the `push` method
fn vector_push_method() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector push method vector: {:?}", &v);
}

// There are two ways to access elements in a vector, either with indexing syntax or the `get` method
fn reading_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];
    // We use the index value of 2 to get the third element, vectors are indexed by number, starting
    // at zero. Also, by using `&` and `[]`, which gives us a reference
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Another way is by using the `get` method with the index passed as an argument, which gives us
    // an `Option<&T>`
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn index_out_of_bounds() {
    let v = vec![1, 2, 3, 4, 5];
    // Commented below produces runtime error of 'index out of bounds'
    // let does_not_exist = &v[100];
    // Below returns `None` without panicking
    let does_not_exist = v.get(100);
    println!("{:?}", &does_not_exist);
}

// When the program has a valid reference, the borrow checker enforces the ownership and borrowing
// rules to ensure this reference and any other references to the contents of the vector remain valid
// Recall the rule that states that you can't have mutable/exclusive and immutable/shared references
// in the same scope. That rule applies to code below, where we hold an immutable/shared reference to the
// first element in a vector and try to add an element to the end, which won't work if also try to
// refer to that element later in the function.
fn vector_borrow_rules() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6); // this works
    let first = &v[0];
    // v.push(6); // this doesn't work
    println!("The first element is: {}", first);
}

// If we want to access each element in a vector one by one, we can iterate through all of the elements
// rather use indexes to access one at a time.
fn iterate_through_vector() {
    let v = vec![100, 32, 57];
    println!("The vector's element values are:");
    // Here we use a `for` loop to get immutable/shared references to each element in a vector of
    // i32 values and print them
    for i in &v {
        println!("\t{}", i);
    }
}

// We can also iterate over mutable/exclusive references to each element in a mutable vector in order
// to make changes to all the elements. The `for` loop will add 50 to each element
fn iterate_through_mut_vector() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable/exclusive reference refers to, we have to use the
        // dereference operator (*) to get to the value in `i` before we can use the `+=` operator.
        *i += 50;
    }
    println!("The vector's values are: {:?}", v);
}

// Before we said that vectors can only store values that are the same type. This can be inconvenient;
// there are definitely use cases for needing to store a list of items of different types.
// Fortunately, the variants of an enum are defined under the same enum type, so when we need to
// store elements of a different type in a vector, we can define and us an enum.
//
// For example, say we want to get values from a row in a spreadsheet in which some of the columns
// in the row contain integers, some floating-point numbers, and some strings. We can define an enum
// whose variants will hold the different value types, and then all the enum variants will be
// considered the same type: that of the enum. Then we can create a vector that holds that enum and
// so, ultimately, holds different types
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// Rust needs to know what types will be in the vector at compile time so it knows exactly how much
// memeory on the heap will be needed to store each element. A secondary advantage is that we can be
// explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type,
// there would be a chance that one or more of the types would cause error with the operations
// performed on the elements of the vector. Using an enum plus a `match` expression means that Rust
// will ensure at compile time that every possible case is handled.
//
// There is one caveat to this technique: if you don't know the exhaustive set of types that the
// program will get at runtime to store in a vector, the enum technique will not work. Instead, you
// can use a trait object, which is covered in another crate.
fn using_enum_for_multi_type_vector() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Multi-type Vector is: {:?}", row);
}

fn main() {
    creat_empty_vector();
    vector_push_method();
    reading_elements_of_vectors();
    index_out_of_bounds();
    vector_borrow_rules();
    iterate_through_vector();
    iterate_through_mut_vector();
    using_enum_for_multi_type_vector();
}
