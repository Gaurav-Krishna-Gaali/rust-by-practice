// Solution

// Add another type, here it is U
struct Point<T, U> {
    x: T,
    // Use type U for y to make it independent of the type of x, so it can be a String in this case
    y: U, 
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}