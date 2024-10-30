fn main() {
    let r;

    {
        let x = 5;
        r = &x;

        // Scope for x ends after this curly brace
    }

    // but r is still valid because it is still in scope, because its scope is larger
    println!("r : {r}");
}

// Below is the correct implementation of the code above

// fn main () {
// let x =5;
// let r=&x;
// println!("r : {r}");
// }
