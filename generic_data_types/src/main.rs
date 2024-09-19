// we are defining two different functions that takes different types of parameters but do the same
// we will then combine these two as generic data type

// function with i32 as parameter
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// function with char as parameter
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![23, 35, 3, 54, 1];
    let result = largest_i32(&number_list);
    println!("The Largest number in the list is : {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
// go to the 2nd part of this file to understand further
