
//  Solution
fn main() {
    let f = true;
    let t = true && false;

    // Add ! before f to make it false therefore equal to t
    assert_eq!(t, !f); 

    println!("Success!");
}
