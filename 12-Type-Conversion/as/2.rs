// Solution
// This will will let the program compile even if there are overflowing literals
#[allow(overflowing_literals)] 
fn main() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;

    println!("Success!")
}