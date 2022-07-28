// Solution
fn main() {
    let decimal = 97.123_f32;

    // The type is u8 since we are converting decimal to u8
    let integer: u8 = decimal as u8; 

    // First convert decimal to integer and then to char
    let c1: char = decimal as u8 as char; 
    let c2 = integer as char;

    // Add + 1 to integer to make the value 98 so that they both match. We could also do 'b' as u8 - 1 to make both 97
    assert_eq!(integer + 1, 'b' as u8);

    println!("Success!")
}
