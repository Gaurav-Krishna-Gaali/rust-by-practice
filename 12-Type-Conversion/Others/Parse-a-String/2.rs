// Solution
use std::str::FromStr;
fn main() {
    //  .parse() to get the int from a string
    let parsed: i32 = "5".parse().unwrap(); 
    //  .parse::<T>() to parse T from a String
    let turbo_parsed = "10".parse::<i32>().unwrap(); 
    //  i32.from_str() to convert a string to an integer
    let from_str = i32::from_str("20").unwrap(); 
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}