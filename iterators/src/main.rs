/*
    The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator
    is responsible for the logic of iterating over each item and determining when the sequence has
    finished. When you use iterators, you don't have to reimplement that logic yourself.

    In Rust, iterators are "lazy", meaning they have no effect until you call methods that consume
    the iterator to use it up. For example, the code below creates an iterator over the items in the
    the vector `v1` by calling the `iter` method defined on `Vec<T>`
*/
fn example_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("{:?}", v1_iter);
    /*
        Once we've created an iterator, we can use it in a variety of ways. Before we used iterators
        with `for` loops to execute some code on each item.

        Here we separate the creation of the iterator from the use of the iterator in the `for` loop
        The iterator is stored in the `v1_iter` variable, and no iteration takes place at that time.
        When the `for` loop is called using the iterator in `v1_inter`, each element in the iterator
        is used in one iteratoion of the loop which print out each value.
    */
    for val in v1_iter {
        println!("Got: {}", val);
    }
    /*
        In languages that don't have iterators, you would likely write this same functionality by
        starting at index 0, using that variable to index into the vector to get a value, and
        incrementing the variable value in a loop intil it reached the total number of items in the
        vector.

        Iterators handle all that logic for you, cutting down on repetetive code you could potentially
        mess up. Iterators give you more flexibility to use the same logic with many different kinds
        of sequences, not just data structures you can index into, like vectors. Let's examine how
        they do that.
    */
}

/*
    The Iterator Trait and the next Method

    All iterators implement the trait named `Iterator` that is defined in the standard library.
    The definition of the trait looks like this...
    `pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }`
    Notice this definition uses some new syntax: `type Item` and `Self::Item`, which are defining a
    "associated type" with this trait. This code says implementing the `Iterator` trait requires that
    you alos define an `Item` type, and this `Item` type is used in the return type of the `next`
    method. In other words, the `Item` type will be the type returned from the iterator.

    The `Iterator` trait only requires implementors to define one method: the `next` method, which
    returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns
    `None`.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        /*
            Note that we need to make `v1_iter` mutable: calling the `next` method on an iterator
            changes internal state that the iterator uses to keep track of where it is in the
            sequence. In other words, this code "consumes", or uses up, the iterator. Each call to
            `next` eats up an item form the iterator. We didn't need to make `v1_iter` mutable in
            the `for` loop above because the loop took ownership of `v1_iter` and made it mutable
            behind the scenes.
        */
        let mut v1_iter = v1.iter();
        /*
            Also note that the values we get from the the calls to `next` are immutable references to
            the values in the vector. The `iter` method produces an iterator over immutable references
            If we want to create an iterator that takes ownership of `v1` and returns owned values,
            we cann call `into_inter` instead of `iter`. Similarly, if want to iterate over mutable
            references, we can call `iter_mut` instead of `iter`
        */
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    /*
        Methods that Consume the Iterator

        The `iterator` trait has a number of different methods with default implementations provided
        by the standard library.

        Methods that call `next` are called "consuming adaptors", because calling them uses up the
        iterator. One example is the `sum` method, which takes ownership of the iterator and iterates
        through the items by repeatedly calling `next`. As it does this it adds each item into a
        running total and returns the the total when the iteration is complete
    */
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

/*
    Methods that Produce Other Iterators

    Other methods defined on the `Iterator` trait, known as "iterator adaptors", allow you to change
    iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors
    to perform complex actions in a readable way. But because all iterators are "Lazy" you have to
    call one of the consuming adaptors to get results form calls to iterator adaptors.
*/
fn iterator_adaptor_with_consumer() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}

fn main() {
    example_iter();
    iterator_adaptor_with_consumer();
}
