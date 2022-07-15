
// Solution
fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
     // Add reference to s1 and make range 0..1 which is 0
    assert_eq!(h, "h");

    let h1 = &s1[3..=5]; 
    // Add ..= to include 5
    assert_eq!(h1, "中");

    println!("Success!");
}
