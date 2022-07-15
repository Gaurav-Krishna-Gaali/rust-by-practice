// A scope is the range within the program for which the item is valid.
//
// Given

// Fix the error with the use of define_x
/*fn main() {
    println!("{}, world", x);
}

fn define_x() {
    let x = "hello";
}*/

// Solution
fn main() {
    let x = define_x();
    println!("{x}, world");
}

fn define_x() -> String {
    "hello".to_string() // Returning the value of x
}
