fn main() {
    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    // Equivqlent to the code snippet above it
    let _s = String::from("initial contents");

    // Updating a stringify!

    // Appending to a String with push_str and push
    let mut s = String::from("Hello");
    s.push_str(" World !");

    // A Better Way
    let mut s1 = String::from("Hello Everyone");
    let s2 = " ! yo ! ";
    s1.push_str(s2);

    // The push_str method appends a string slice (&str) to the end of a String. It takes a string slice as an argument.
    // The push method appends a single character to the end of a String. It takes a char as an argument.

    let ss1 = String::from("Hello, ");
    let ss2 = String::from("world!");

    // note s1 has been moved here and can no longer be used
    let ss3 = ss1 + &ss2;

    // Concatenating Strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Methods for Iterating Over Strings
    for c in "Hello".chars() {
        println!("{c}");
    }

    // the bytes method returns each raw byte
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // But be sure to remember that valid Unicode scalar values may be made up of more than one byte.
    // The good news is that the standard library offers a lot of functionality built off the String and &str types to help handle these complex situations correctly. Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.
}
