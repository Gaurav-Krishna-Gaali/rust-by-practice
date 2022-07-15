
// Solution
fn main() {
     // Make var mutable
    let mut s = String::from("hello, ");   
    // Make a mutable reference
    let s1 = &mut s;

    s1.push_str("world");

    println!("Success!");
}
