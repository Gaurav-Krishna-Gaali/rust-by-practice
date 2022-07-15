
// Solution
fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    //  mutable reference to s
    let p = &mut s; 
    
    p.push_str("world");

    println!("Success!");
}
