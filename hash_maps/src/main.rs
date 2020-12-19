// The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`. It does this
// via a "Hashing Function", which determines how it places these keys and values into memory. Many
// programming languages support this kind of data structure, but they often us a different name.
//
// Hash maps are useful when you want to lookup data not by using an index, as you can with vectors,
// but by using a key that can be of any type. For example, in a game, you could keep track of each
// team's score in a has map in which each key is the team's name and the values are each team's
// score. Given a team name, you can retrieve the score.

// You can create an empty hash map with `new` and add elements with `insert`. In the example below
// we're keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team
// starts with 10 points, and the Yellow team starts with 50.
// Need to bring HashMap namespace into scope
use std::collections::HashMap;

// Like vectors, hash maps store their data on the heap. Keys must be of the same type, and values
// must also be the same type
fn create_new_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores HashMap is: {:?}", &scores);
}

// Another way of constructing a hash map is by using iterators and the `collect` method on a vector
// of tuples, where each tuple consists of a key and its value.
fn create_hashmap_with_collect() {
    // The `collect` method gathers data into a number of collection types, including `HashMap`. For
    // example, if we has the team names and initial scores in two separate vectors, we could use the
    // `zip` method to create a vector of tuples where "Blue" is paired with 10, and "Yellow" is
    // paired with 50. Then we could use `collect` method to turn that vector of tuples into a
    // hash map like below.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // The type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into
    // many different data structures and Rust doesn't know which you want unless you specify. For
    // the parameters for the key and value types, however, we use underscores, and Rust can infer
    // the types that the hash map contains based on the types of the data in the vectors.
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores Hashmap with Collect is: {:?}", &scores)
}

// For types that implement the `Copy` trait, like i32, the values are copied into the hashmap. For
// owned values like `String`, the values will be moved and the hash map will be the owner of those
// values.
fn hashmaps_and_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // `field_name` and `field_value` are invalid at this point
    // if we instead insert references to values into the hashmap, the values won't be moved into the
    // hashmap. The values that the references point to must valid for at least as long as the hasmap
    // is valid.
    println!("{:?}", &map);
}

// We can get a value out of the hashmap by providing it key to the `get` method
fn accessing_hashmap_values() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // Here, `score` will have the value that's associated with the blue team, and the result will
    // be `Some(&10)`
    let score = scores.get(&team_name);
    match score {
        // The result is wrapped in `Some` because `get` returns an `Option<&V>`
        Some(x) => println!("The Blue team's score is: {}", x),
        // If there's no value for that key name in the hashmap, `get` will return `None`
        None => println!("That team doesn't have a score"),
    }
}

// We can iterate over each key/value pair in a hashmap in a similar as we do with vectors, using a
// `for` loop:
fn iterate_hashmap_with_for() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

// Although the number of keys and values is growable, each key can only have one value associated
// with it at a time. When you to change the data in a hashmap, you have to decide how to handle the
// case when a key already has a value assigned. You could replace the old value with the new value,
// completely disregarding the old value. You could keep the old value and ignore the new value, only
// adding the new value if the key doesn't already have a value. Or you could combine the old value
// and the new value.
//
// If we instert a key and a value into a hashmap and then insert that same key with a different
// value, the value associated with that key will be replaced. Even though the code below calls
// `insert` twice, the hashmap will only contain one key/value pair because we're inserting the for
// the Blue team's key both times
fn overwriting_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // This code will print `{"Blue": 25}`, the original value of 10 have been overwritten
    println!("{:?}", scores);
}

// It's common to check whether a particular key has value and , if doesn't, insert a value for it.
// Hashmaps have a special API for this called `entry` that takes the key you want to check as a
// parameter.
fn insert_if_key_not_exist() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // The `or_insert` method on `Entry` is defined to return a mutable/exclusive reference to the value
    // for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the
    // new value for this key and reutrns a mutable/exclusive reference to the new value. This
    // technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely
    // with the borrow checker.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // This code will print `{"Blue": 10, "Yellow": 50}`. The first call to `entry` will insert the
    // key for the Yellow team with the value 50 because the Yellow team doesn't have a value already
    // The second call to `entry` will not change the hashmap because the Blue team already has the
    // value 10
    println!("{:?}", scores);
}

// Another common use case is for hashmaps to lookup a key's value and then update it based on the
// old value. For instance, below shows code that counts how many time each word appears in some
// text. We use a hashmap with the words as keys and increment the value to keep track of how many
// times we've seen that word. If it's the first time we've seen the word, we'll first insert the
// value 0
fn update_value_based_on_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // The `or_insert` method actually returns a mutable/exclusive reference (`&mut v`) to the
        // value for this key. Here we store the mutable/exclusive reference in the `count` variable
        // so in order to assign to that value, we must first dereference `count` use the
        // asterisk (*).
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // The mutable/exclusive reference goes out of scope at the end of the `for` loop, here. So
        // all of these changes are safe and allowed by the barrowing rules.
    }
    // This code will print `{"world": 2, "wonderful": 1, "hello": 1}`
    println!("{:?}", map);
}

// Hashing Functions
//
// By default, `HashMap` uses a "cryptographically strong" hashing function that can provide
// resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available
// but the trade-off for better security that come with the drop in performance is worth it. If you
// profile your code and find that the default hash function is too slow for your purposes, you can
// switch to another function by specifying a different "hasher". A hasher is a type that implements
// the `BuildHasher` trait.

fn main() {
    create_new_hashmap();
    create_hashmap_with_collect();
    hashmaps_and_ownership();
    accessing_hashmap_values();
    iterate_hashmap_with_for();
    overwriting_value();
    insert_if_key_not_exist();
    update_value_based_on_old_value();
}
