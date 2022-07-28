// Solution
fn main() {
    // Use String::from() to convert the string literal to a String
    let mut s: String = String::from("hello, "); 
    // Remove to_string() because push_str expects a string literal
    s.push_str("world"); 
    // .push() only adds a char so we give ! as a parameter
    s.push('!'); 

    // Cloning s so the ownership of s doesn't move and we can use it in assert_eq!
    move_ownership(s.clone()); 

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) { 
    println!("ownership of \"{}\" is moved here!", s)
}