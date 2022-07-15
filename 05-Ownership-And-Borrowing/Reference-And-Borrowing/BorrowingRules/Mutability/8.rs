
// Solution
fn main() {
    // Add mut to make s mutable
    let mut s = String::from("hello, "); 

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
