/*
    Rust closures are anonymous functions you can save in a variable or pass as arguments to other
    functions. You can create the closure in one pklace and then call the closure to evaluate it in
    a different conext. Unlike functions, closure can capture value from the scope in which they're
    defined. We'll demonstrate how these closure features allow for code reuse and behavior
    customization.
*/

/*
    Creating an Abstraction of Behavior with Closures

    Let's work on an example of a situation in which it's useful to store a closure to be executed
    later. Along the way, we'll talk about the syntax of closures, type inference, and traits.

    Consider this hypothetical situation: we work at a startup that's making an app to generate
    custom exercise workout plans. The backend is written in Rust, and the algorithm that generates
    the workout plan takes into account many factors, such as the app user's age, body mass index,
    exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm
    used isn't important in this example; what's important is that this calculation takes a few
    seconds. We want to call this algorithm only when we need to and only call it once so we don't
    make the user wait more than necessary.

    We'll simulate calling this hypothetical algorithm with this function...
    `simulated_expensive_calculation`, which will `calculating slowly...`, wait for 2 seconds, and
    then return whatever number we passed in.
*/

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

/*
    Next is the `main` function, which contains the parts of the workout app important for this
    example. This function represents the code that the app will call when a user asks for a workout
    plan. Because the interaction with the app's frontend isn't relevent to the use of closures,
    we'll hardcode values representing inputs to our program and print the outputs

    The required inputs are these:
    - An intensity number from the user, which is specified when they requrest a workout to indicate
      whether they want a low-intensity workout or a high-intensity workout
    - A random number that will generate some variety in the the workout plans

    The output will be the recommend workout plan
*/
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

/*
    We've hardcoded the variable `simulated_user_specified_value` as 10 and the variable
    `simulated_random_number` as 7 for the sake of simplicity; in an actual program, we'd get
    intensity from the app frontend, and we'd use `rand` crate to generate a random number, as we
    did in the `guessing_game` crate. The `main` function calls a `generate_workout` function with
    the simulated input values.

    Now that we have the context, let's get to the algorithm. The function `generate_workout` contains
    the business logic of the app that we're most concerned with in this example. The rest of the
    code changes in this example will be made to this function
*/
#[allow(dead_code)]
fn generate_workout_old1(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

/*
    Refactoring Using Functions

    We could restructure the workout program in many ways. First, we'll try extracting the duplicated
    call to the `simulated_expensive_calculation` function into a variable

*/
#[allow(dead_code)]
fn generate_workout_old2(intensity: u32, random_number: u32) {
    /*
        This change unifies call the calls to `simulated_expensive_calculation` and solves the
        problem of the first `if` block unnecessarily calling the function twice. Unfortunately, we're
        now calling this function and waiting for the result in all cases
    */
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            // including here where we're not using the result value at all
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
    /*
        We want to define code in one place in our program, but only "execute" that code where we
        actually need the result. This is a use case for closures
    */
}

/*
    Refactoring with Closures to Store Code

    Instead of always calling the `simulated_expensive_calculation` function before the `if` blocks,
    we can define a closure and store the closure in a variable rather than storeing the result of
    the function call. We can actually move the whole body of `simulated_expensive_calculation` within
    the closure
*/
#[allow(dead_code)]
fn generate_workout_old3(intensity: u32, random_number: u32) {
    /*
        The closure definition comes after the = to assign it to the variable `expensive_closure`.
        To define a closure, we start with a pair of vertical pipes (|), inside which we specify
        the parameters to the closure; this syntax was chosen because of its similarity to closure
        definitions in Smalltalk and Ruby. This closure has one parameter named `num`: if we had
        more than one parameter, we would separate them with commas, like `|param1, param2|`
    */
    let expensive_closure = |num| {
        /*
            After the parameters, we place curly brackets that hold the body of the closure--these
            are optional if the closure body is a single expression. The end of the closure after
            the curly brackets, needs a semicolon to complete the `let` statement. The value returned
            from the last line in this closure body (`num`) will be the value return from the closure
            when it's called, because that line doesn't end in a semicolon; just as function bodies.
        */
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    /*
        Note that this `let` statement means `expensive_closure` contains the "definition" of an
        anonymous function, not the "resulting value" of calling the anonymous function. Recall that
        that we are using a closure here because we want to define the code to call at one point,
        store the code, and call it later at a later point; the code we want to call is now stored
        in the `expensive_closure` variable.
    */

    if intensity < 25 {
        /*
            We've reintroduced one of the problems from the last example: we're still calling the
            closure twice here, and make the user wait twice as long as they need to. We could fix
            this problem by creating a variable local to this block to hold the result of calling
            the closure, but closures provide us with another solution. We'll look at that here in a
            bit, but first let's look at why there aren't any type annotation involved with closures
        */
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

/*
    Closure Type Inference and Annotation

    Closures don't require you to annotate the types of the parameters or the return value like `fn`
    functions do. Type annotations are required on functions because they're part of an explicit
    interface exposed to users. Defining this interface rigidly is important for ensuring that
    everyone agrees on what types of values a function uses and returns. But closures aren't used in
    an exposed interface like this: they're stored in variables and used without naming them and
    exposing them to users of our library.

    Closures are usually short and relevant only within a narraow context rather than in any arbitrary
    scenario. Within these contexts, the compiler is reliably able to infer the types of the parameters
    and the return type, similar to how it's able to infer types of most variables.

    Making programmers annotate the types in these small, anonymous functions would be annoying and
    largely redundant with the information the compiler already has available.

    As with variables, we can add type annotations if we want to increase the explicitness and clarity
    at the cost of being more verbose than is strictly necessary. But here is how it would look
    ``let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
     };``

    Closure definitions will have one concrete type inferred for each of their parameters and for
    their return value.
*/

/*
    Storing Closures Using Generic Parameters and the Fn Traits

    Let's return to our workout generation app. Our code before was still calling the expensive
    calculation closer more times than it needed to. One option to solve this issue is to save the
    result of the expensive closure in a variable for reuse and use the variable in each place we
    need the result, instead of calling the closure again. However, this method could result in a
    lot of repeated code.

    Fortunately there is another solution available to us. We can create a struct that will hold the
    closure and the resulting value of calling the closure. The struct will execute the closure only
    if if we need the resulting value, and it will cache the resulting value so the rest of our code
    doesn't have to be responsible for saving and reusing the result. You may know this pattern as
    "memoization" or "lazy evaluation".

    To make a struct that holds a closure, we need to specify the type of of the closure, because a
    struct definition need to know the types in each of its fields. Each closure instance has its
    own unique anonymous type: that is, even if two closures have the same signature, their types are
    still considered different. To define stucts, enums, or function parameters that use closures, we
    use generics and trait bounds.

    The `Fn` traits are provided by the standard library. All closures implement at least one of the
    traits: `Fn`, `FnMut`, or `FnOnce`, we'll discuss the differences later on.

    We add types to the `Fn` trait bound to represent the types of the parameters and return values
    the closures must have to match this trait bound. In this case, our closure has a parameter of
    type u32 and returns a u32, so the trait bound we specify is...
                `Fn(u32) -> u32`
*/
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    /*
        This struct has a `calculation` field of the generic type `T`. The trait bounds on `T` specify
        that it's closure by using the `Fn` trait. Any closure we to store in the calculation field
        must have one u32 parameter (specified within the parenthesis after `Fn`) and must return a
        u32 (specified after the ->)
    */
    calculation: T,
    /*
        The `value` field is of type `Option<u32>`. Before we execute the closure, `value` will be
        `None`. When code using this struct asks for the "result" of the closure, the `Cacher` will
        execute the closure at that time and store the result within a `Some` variant in the `value`
        field. Then if the code asks for the result of the closure again, instead of executing the
        closure again, the `Cacher` will return the result held in the `Some` variant.
    */
    value: Option<u32>,
}

/*
    We want `Cacher` to manage the struct fields' values rather than letting the calling code potentially
    change the values in these fields directly, so these fields are private.
*/
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    /*
        The `Cacher::new` function takes a generic parameter `T`, which we've defined as having the
        same trait bound as the `Cacher` struct. Then `Cacher::new` returns a `Cacher` instance that
        holds the closure specified in the `calculation` field and a `None` value in the `value`
        field, because we haven't executed the closure yet.
    */
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    /*
        When the calling code needs the result of evaluating the closure, instead of calling the
        closure directly, it will call the `value` method. This method checks whether we already have
        a resulting value in `self.value` in `Some`; if we do, it returns the value within the `Some`
        without executing the closure again.

        If `self.value` is `None`, the code calls the closure stored in the `self.calculation` saves
        the result in `self.value` for future use and return the value as well.
    */
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/*
    Here is how can use the `Cacher` struct in the function `generate_workout`
*/
fn generate_workout(intensity: u32, random_number: u32) {
    /*
        Instead of saving the closure in a variable directly, we save an instance of `Cacher` that
        holds the closure. Then, in each place we the result, we call the `value` method on the
        `Cacher` instance. We can call the `value` method as many times as we want, or not call it
        at all, and the expensive calculation will be run a maximum of once.
    */
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

/*
    Capturing the Environment with Closures

    In the workout generator example, we only used closures as inline anonymous functions. However,
    closures have an additional capability that functions don't have: they can capture their
    environment and access variables from the scope in which they're defiined.
*/
