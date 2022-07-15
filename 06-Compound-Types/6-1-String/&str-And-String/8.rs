
// Solution
fn main() {
    // Add a reference
    let s =  "hello, world".to_string();
    let s1: &str = &s; // Add & to make it &str
    
    // Make s an &str too
    let s = "hello, world";
    let s1: &str = s;

    println!("Success!");
}
