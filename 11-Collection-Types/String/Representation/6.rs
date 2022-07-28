// Solution
fn main() {
    // Create s with capacity of 25
    let mut s = String::with_capacity(25); 

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}