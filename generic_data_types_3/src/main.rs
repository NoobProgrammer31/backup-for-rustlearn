// Some, which holds one value of type T, and a None variant that doesn’t hold any value
enum Option<T> {
    Some(T),
    None,
}

// Multiple generic types with enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generic Data Types In Method Definitions
// struct Point<T> {
//     // x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// we can restrict methods to work on specific data types too as follows
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// Example of more depth on generics and methods
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let p = Point { x: 5, y: 7 };
    // println!("p.x is {}", p.x());
}
