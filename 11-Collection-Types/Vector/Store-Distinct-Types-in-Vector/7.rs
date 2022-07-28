// Solution
#[derive(Debug)]
#[derive(PartialEq)] // Add partial eq trait for it to be compared
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // Make new vector and add two elements that are the two variants of the IpAddr Enum
    let v : Vec<IpAddr>= vec!(IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string()));
    
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}