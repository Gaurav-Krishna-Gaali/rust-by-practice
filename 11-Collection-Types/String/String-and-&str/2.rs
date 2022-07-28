// Solution
fn main() {  
    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; // Reference to String s to make it &str
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[..5]; // Slice type from String s till hello
    assert_eq!(slice2, "hello");
 
    let mut slice3: String = "hello, world".to_string(); // Mutable String and use to_string() function to make the literal a String
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!")
 }