//  Solution
fn main() {
    println!("{argument}", argument = "test");

    // define value of name
    assert_eq!(format!("{name}{}", 1, name="2"), "21"); 
    // Add the format of how the string will look like
    assert_eq!(format!("{a} {c} {b}",a = "a", b = 'b', c = 3 ), "a 3 b"); 
    
    // Change 1 to 0 and put 2 before the named declaration
    println!("{abc} {0}", 2, abc = "def"); 

    println!("Success!")
}
