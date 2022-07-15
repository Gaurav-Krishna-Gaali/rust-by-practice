// Solution

fn main() {
    let mut s = String::from("hello");

    // mut removed from s, so it is immutable
    let r1 = &s; 
    let r2 = &s; 

    println!("{}, {}", r1, r2);

    println!("Success!");
}

