// This program will take the width and height of a rectangle specified in pixels and calculate the
// area of the rectangle.
//
// We'll start with single variables, and then refactor the program until we're using structs
// instead

fn main() {
    main_rectangle1();
    main_rectangle_tuple();
    main_rectangle_structs();
}

fn main_rectangle1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle without structs is {} square pixels.",
        area1(width1, height1)
    );
}

// The width and height are related to each other because together they describe one rectangle
// This function is supposed to calculate the area of one rectangle, but this function has two
// parameters. The parameters are related, but that's not expressed anywhere in this program. It
// would be more readable and more manageable to group width and height together.
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// We can use tuples to refactor above
fn main_rectangle_tuple() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle with tuples is {} square pixels.",
        area_tuple(rect1)
    );
}

// In one way, this program is better. Tuples let us add a bit of structure, and we're now passing
// just one argument. But in another way, this version is less clear: tuples don't name their
// elements, so our calculation has become more confusing because we have to index into the parts
// of the tuple.
//
// It doesn't matter if we mix up width and height for the area calculation, but if we want to draw
// the rectangle on the screen, it would matter. We have to keep in mind that `width` is the tuple
// index 0 and height is tuple index 1. If someone else worked on this code, they wouid have to
// figure this out and keep it mind as well. It would be easy to forget or mix up these values and
// cause errors, because we haven't conveyed the meaning of our data in our code.
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Now lets use structs to add meaning by labeling the data. We can transform the tuple we're using
// into a data type with a name for the whole as well as names for the parts
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main_rectangle_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle with structs is {} square pixels.",
        area_structs(&rect1)
    );
    println!("rect1 is {:?}", rect1);
}

// this function is now defined with one parameter, whose type is a immutable borrow of the struct
// `Rectangle` instance.
fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
