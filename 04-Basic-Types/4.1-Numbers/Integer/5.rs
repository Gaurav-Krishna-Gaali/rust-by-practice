
// Solution
fn main() {
    // Change 251 to 247 as it  does not make an overflow error
    let v1 = 247_u8 + 8; 
    // Change i8 to u8 and change 251 to 247 so it doesn't throw errro
    let v2 = u8::checked_add(247, 8).unwrap(); 
    println!("{v1}, {v2}");
}

