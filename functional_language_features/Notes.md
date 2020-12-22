**Functional Language Features: Iterators and Closures**

Rust's design has taken inspiration from many existing languages and techniques, and one significant
influence is *functional programming*. Programming in a functional style often includes using functions
as values by passing them in arguments, returning them from other functions, assigning them to variables
for later execution, and so forth.

In this chapter, we won't debate the issue of what functional programming is or is not but will 
instead discuss some features of rust that are similar to features in many languages often referred
to as functional.

More specifically, we'll cover:

- *Closures*, a function-like construct you can store in a variable
- *Iterators*, a way of processing a series of elements
- How to use these two feature to improve the I/O project, `minigrep`
- The performance of these two features

Other Rust features, such as pattern matching and enums, which we've covered in other crates, are
influenced by the functional style as well. Mastering closures and iterators is an important part
of writing idiomatic, fast Rust code, so we'll devote this entire crate to them
