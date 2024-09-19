// To define the generic largest function, we place type name declarations inside angle brackets, <>, between
// the name of the function and the parameter list, like this:
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// for now this function will throw an error
// because this "<" can't work on all possible types that T could be

// defining generics with structs
// x and y can contain only a certain data at a time
struct Point<T> {
    x: T,
    y: T,
}

// to allow different data types in one struct we use T and U as follows
struct Diff_point<T, U> {
    a: T,
    b: U,
}
// now a adn b can hold different data types

fn main() {
    let both_integer = Diff_point { a: 5, b: 10 };
    let both_float = Diff_point { a: 1.0, b: 4.0 };
    let integer_and_float = Diff_point { a: 5, b: 4.0 };

    let integer = Point { x: 2, y: 4 };
    let float = Point { x: 1.0, y: 2.0 };

    let number_list = vec![34, 43, 56, 33, 1, 3];
    let result = largest(&number_list);
    println!("The largest number is: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is: {result}");
}
