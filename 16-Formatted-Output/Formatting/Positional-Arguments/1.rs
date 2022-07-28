// Solution
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 21 because index starts at 0
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    // Just make assert work
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112"); 
    println!("Success!");
}