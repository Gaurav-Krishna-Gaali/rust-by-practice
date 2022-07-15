// Diverging functions never return to the caller, so they may be used in place where a value of
// any type is expected
//
// Given
//
/*fn main() {
    // FILL in the blank
    let b = __;

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}*/

// Solution
fn main() {
// FILL in the blank
    let b = false; // Change to false

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
