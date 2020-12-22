// The bodies of test functions typically perform 3 actions
// 1) Setup an needed data or state
// 2) Run the code you want to test
// 3) Assert the results are what you expect

// At its simplest, a test in Rust is a function that's annotated with the `test` attribute
// add `#[test]` on the line before `fn`
// When run tests with `cargo test` command, Rust builds a test runner binary that runs the
// functions annotated with the `#[test]` attribute, and reports on whether each test function passes
// or fails

// When make a new library project with cargo, a test module with a test function in it is automatically
// generated for us.

fn main() {
    println!("Hello, world!");
}
