// Rust variables are immutable by default. When a variable is immutable, once a value is bound to a
// a name, you can't change that value.
fn main() {
    // the `mut` declaration allows us to change `x`s value
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    shadowing();
    shadowing2();
}

#[allow(unused)] // just so compiler won't give us a warning
const MAX_POINTS: u32 = 100_000;

// Shadowing is defferent than marking a variable as `mut`, because we will get a compile time error
// if we forget the keyword `let` and try to assign a value to `x` after the initial binding.
// By using `let`, we can perform a few transformations on a value but have the variable be immutable
fn shadowing() {
    // first binds `x` to a value of 5
    let x = 5;
    // then it shadows `x` by repeating `let x =`, taking the original value and adding 1 to it so
    // the of value of `x` becomes 6
    let x = x + 1;
    // shadow `x` again by multiplying the previous value by 2, to give a final `x` value of 12
    let x = x * 2;
    println!("[Shadowing] The value of x is: {}", x);
}

// The other difference is that because we're effectively creating a new variable when we use the
// `let` keyword again, we can change the type of the value but reuse the same variable name.
// Say we want to show user how many spaces they want between some text by inputing space chars,
// but what we really want to store that input as a number.
fn shadowing2() {
    let spaces = "   ";

    // Below won't compile
    // spaces = spaces.len();
    let spaces = spaces.len();
    println!("There are {} spaces.", spaces);
}
