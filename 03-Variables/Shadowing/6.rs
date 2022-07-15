// Solution

fn main() {
    let mut x: i32 = 1;
    x = 7;
    
    let x = x;
    // Comment out this line as we shadowed and made x immutable
    // x += 3; 

    let y = 4;

    let y = "I can also be bound to text!:";

    println!("Success");
}
