
// Solution
fn main() {
    let s = "你好，世界";
    // Change range to ..3 because these characters take 3 bytes
    let slice = &s[..3]; 
    assert!(slice == "你");

    println!("Success!");
}
