// Rust is a "statically typed" language, which means that it must know the types of all variables
// at compile time. The compiler can usually infer what type we want to use based on the value and
// how we use it.
fn main() {
    numeric_operations();
    char_type();
    tuple_type();
    accessing_array_elements();
    invalid_array_element_access();
}

// A "scalar" type represents a single value. Rust has 4 primary scalar types: integers,
// floating-point numbers, Booleans, and characters.
fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder/modulus
    let remainder = 43 % 5;
    println!("Sum is: {}", sum);
    println!("Difference is: {}", difference);
    println!("Product is: {}", product);
    println!("Quotient is: {}", quotient);
    println!("Remainder is: {}", remainder);
}

// Rust's `char` type is 4 bytes in size and represents a Unicode Scalar Value, which means that it
// can represent more than ASCII.
fn char_type() {
    // use single quotes for characters and double quotes for strings
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let length: usize = heart_eyed_cat.len_utf8();
    println!("{} {} {}", c, z, heart_eyed_cat);
    println!("The length of 'heart_eyed_cat' is: {} bytes", length);
}

// A tuple is a general way of grouping together a number of values with a variety of types into one
// compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size
fn tuple_type() {
    // We create a tuple by writing a comma-separated list values inside parenthesis. Each position
    // in the tuple has a type, and the types of the different values in the tuple don't have to be
    // the same. Here we have added optional type annotations
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a
    // tuple value. It breaks the single tuple into 3 parts. The underscore in front of `x` and `z`
    // tells the Rust compiler that we know that we won't be using those variables
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    // In addition to destructuring through pattern matching, we can access a tuple element directly
    // by using a period (.) followed by the index of the value we want to access.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of 'five_hundred' is: {}", five_hundred);
    println!("The value of 'six_point_four' is: {}", six_point_four);
    println!("The value of 'one' is: {}", one);
}

// Unlike a tuple, every element in an array must have the same type. They do have a fixed length
// like the tuple does
fn _array_type() {
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // You would write an array's type by using square brackets, and within the brackets include the
    // type of each element, a semicolon, and then the number of elements in the array.
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // Writing an array's type like above is similar to an alternative syntax for initializing an
    // array: if you want to create an that contains the same value of each element, you can specify
    // the initial value, followed by a semicolon, and them the length of the array in square
    // brackets
    let _x = [3; 5];
    // `_x` and `_y` are the same
    let _y = [3, 3, 3, 3, 3];
}

// An array is a single chunk of memory allocated on the stack. You can access elements of an array
// using indexing
fn accessing_array_elements() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first is: {}", first);
    println!("second is: {}", second);
}

// In many low-level languages, this kind of check is not done, and when you provide an incorrect
// index, invalid memory can be accessed. Rust protects you against this kind of error by immediately
// exiting instead of allowing the memory access and continuing.
fn invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    // below will compile but will produce a runtime error because the index is too big.
    let element = a[index];
    println!("The value of element is: {}", element);
}
