// Solution
fn main() {
    let s1 = "hello";
    // Make(format) string to include s1 and equal hello, world!
    let s = format!("{}, world!", s1); 
    assert_eq!(s, "hello, world!");
}
