// "Methods" are similar to function: they're declared with the `fn` keyword and their name, they
// can have parameters and a return value, and they contain some code that is run when they're called
// from somewhere else. However, methods are different from functions in that they're defined within
// the context of a struct (or enum, or trait object), and their first parameter is always `self`,
// which represents the instance of the struct the method is being called on.

// Defining Methods
//
// Let's change the `area` function has a `Rectangle` instance as a parameter and instead make and
// `area` method on the `Rectangle` struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function with the context of `Rectangle`, we start an `impl` (implementation) block
impl Rectangle {
    // Then we move the `area` function with the `impl` curly brackets and change the first (and in
    // this case only) parameter to be `self`. We use `&self` instead of `rectangle: &Rectangle`
    // because Rust knows the type of `self` is `Rectangle` due to the methods being inside the
    // `impl Rectangel` context. Note that we still need to use the `&` before `self`, just as we
    // did in `&Rectangle`. Methods can take ownership of `self`, borrow `self` immutably as we've
    // done here, or borrow `self` mutably, just as they can with any other parameter.
    //
    // We've chosen `&self` here for the same reason we use `&Rectangle` in the function version in
    // the rectangle crate: we don't want to take ownership, and we just want to read the data in
    // the struct, not write to it. If we wanted to change the instance inside this method we'd use
    // `&mut self` as the first parameter instead.
    //
    // Having this method take ownership of the instance by using `self` is rare, it is usually done
    // when the method transforms `self` into something else and you wnat to prevent the caller from
    // using the original instance after it has been transformed.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // use method syntax here to call the `area()` method on our `Rectangle` struct instance.
        // The method syntax goes after an instance: we add a dot (.) followed the method name,
        // parentheses, and any arguments
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // to call the associated function we use this syntax instead of method syntax
    println!("{:?}", Rectangle::square(3));
}

// Another useful feature of `impl` blocks in that we're allowd to define function within `impl`
// blocks that don't take `self` as a parameter. These are called "associated functions" because
// they're associated with the struct. They're still functions, not methods, beause they don't
// have an instance of the struct to work with. For example, `String::from()` is an associated
// function.
//
// Associated functions are used for constructors that will return a new instance of the struct.
// For example, we could provide an associated function that would have one dimension paramteter
// use that as both the width and the height, thus making it easier to crate a square `Rectangle`
// rather than having to specify the same value twice
//
// It's perfectly acceptable to multiple `impl` blocks which are not connected by curly brackets
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
