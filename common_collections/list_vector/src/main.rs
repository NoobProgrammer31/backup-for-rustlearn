fn main() {
    //  creating a new empty vector
    //  let v: Vec<i32> = Vec::new();

    //  one more way to initialise vecctors is
    //  let v = vec![1, 2, 3];

    //  modifying a vector
    //  let mut v = Vec::new();
    //  v.push(5);
    //  v.push(4);
    //  v.push(3);
    //  v.push(2);

    //  Reading Elements from a vector

    let v = vec![1, 2, 3, 4, 5];

    // reading element by referencing
    let third: &i32 = &v[2];
    println!("Third Element is : {third}");

    // Reading element by get method
    let thirdy: Option<&i32> = v.get(2);
    match thirdy {
        Some(thirdy) => println!("The third element is: {thirdy}"),
        None => println!("There is no third element."),
    }

    // Iterating Over the Values in a Vector
    let a = vec![100, 32, 44];
    for i in &a {
        println!("{i}");
    }

    // Iterating Over the Mutable Values in a Vector
    let mut x = vec![100, 32, 44];
    for i in &mut x {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Text(String::from("Hello")),
        Spreadsheetcell::Float(10.12),
    ];
}
