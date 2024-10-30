// Generic Lifetimes in Functions
// write a function that returns the longer of two string slices. This function will take two string slices and return a single string slice.

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// Note that we want the function to take string slices, which are references, rather than strings
// because we don’t want the longest function to take ownership of its parameters.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we can’t look at the scopes as we did in last program to determine whether the reference we return will always be valid
// The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y
// To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so
// the borrow checker can perform its analysis

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
