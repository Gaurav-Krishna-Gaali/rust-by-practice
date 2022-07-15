
// Solution(s)
fn main() {
    // Methiod One
    // Using a reference
    let x = String::from("hello, world");
    let y = &x;

    // Method Two
    // slicing to get the entire string
    let x = String::from("hello, world");
    let y = &x[..];

    // Method Three
    // copy string 
    let x = String::from("hello, world");
    let y = x.clone();


    println!("{},{}",x,y);
}
