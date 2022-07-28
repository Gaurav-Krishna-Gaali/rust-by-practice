// Solution
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // Make format method that takes a mutable reference to the formatter 
        write!(f, "The point is ({}, {})", self.x, self.y) // Write the string literal to the default formatter
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");  // Use to_string() method
    assert_eq!(format!("{}", origin), "The point is (0, 0)"); // Just format a string to match the other one or convert origin to string

    println!("Success!")
}