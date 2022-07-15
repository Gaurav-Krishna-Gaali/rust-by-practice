//  Solution

// A scope is simply the range within the program for which the item is valid

// I removed variable y because it is no longer a valid variable

fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {x} and value of y is {y}");
    }
    println!("The value of x is {x}"); 
}
