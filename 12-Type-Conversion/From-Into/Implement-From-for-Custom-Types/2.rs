// Solution
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // Implement a method that returns a Number instance
    fn from(num: i32) -> Number {
        Number {
            value: num,
        }
    }
}

// FILL in the blanks
fn main() {
    let num = Number::from(30); // Use from trait to get a Number from 30
    assert_eq!(num.value, 30);

    let num: Number = Number {value: 30}; // Make a nNumber instance manually
    assert_eq!(num.value, 30);

    println!("Success!")
}