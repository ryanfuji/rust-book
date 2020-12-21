// Removing Duplication by Extracting a Function
//
// Consider a short program that finds the largest number in a list
fn find_largest_number_in_list() {
    // This code stores a list of integers in the variable `number_list` and...
    let number_list = vec![34, 50, 25, 100, 65];
    // places the first number in the list in a variable named `largest`
    let mut largest = number_list[0];
    // Then it iterates through all the numbers in the list
    for number in number_list {
        // and if the current number is greater than the number stored in `largest`
        if number > largest {
            // it repluaces the number in that variable. However if the current number is less than
            // or equal to the largest number seen so far, that variable doesn't change, and the code
            // movers on to the next number in the list.
            largest = number;
        }
    }
    // After considering all the numbers in the list, `larget` should hold the largest number
    println!("The largest number is {}", largest);
}

// To find the largest number in two different lists of numbers, we can duplicate the code above
// and use the same logic two different places in the program
fn find_larget_number_2_lists() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // Although this code works, duplicating code is tecdious and error prone. We also have to update
    // code in multiple places when we want to change it.
    //
    // To elminitate this duplication, we can create an abstraction by defining a function that
    // operates on any list of integers given to it in a parameter. This solutions makes our code
    // clearer and less us express the concept of finding the largest number in a list abstractly.
}

// We extracted the code that finds the largest number into a function named `largest`. Unlike above
// which can find the largest number in only one particular list, this function can find the
// largest number in two different lists.
//
// This function has a parameter called `list`, which represents any concreate slice of i32 values
// that we might pass into the function. As a result, wehn we call the function, the code runs on
// the specific values that we pass in.
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    find_largest_number_in_list();
    find_larget_number_2_lists();
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
