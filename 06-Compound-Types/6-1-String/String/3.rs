// Solution
fn main() {
    // Make String type from an empty &str
    let mut s = String::from(""); 
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

