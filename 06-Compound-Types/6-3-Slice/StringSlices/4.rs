
// Solution
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
     // Same as 0..2 but without zero is valid
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
