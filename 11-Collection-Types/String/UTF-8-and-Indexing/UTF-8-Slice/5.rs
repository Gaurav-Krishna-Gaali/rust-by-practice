// Solution
fn main() {
    let mut s = String::new();
    s.push_str("hello"); // Add hello to s so it is equal to s1 in assert_eq!

    let v = vec![104, 101, 108, 108, 111];

    // Create String s1 from utf-8 bytes(v) and use .unwrap() and .expect to handle err
    let s1 = String::from_utf8(v).expect("Error while getting String from bytes");
    
    assert_eq!(s, s1);

    println!("Success!")
}