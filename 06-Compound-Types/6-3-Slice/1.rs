
// Solution
fn main() {
    let arr = [1, 2, 3];
    //  we are expecting a slice, so  Add ampersand to indicate 
    let s1: &[i32] = &arr[0..2]; 

    // Make str &str and remove as str
    let s2: &str = "hello, world"; 

    println!("Success!");
}

