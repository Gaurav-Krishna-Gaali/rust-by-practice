// Solution
#[allow(overflowing_literals)] // Add this to compile the program with overflowing literals
fn main() {
    assert_eq!(1000 as u16, 1000); // 1000 because u16 can contain values till 1024

    assert_eq!(1000 as u8, 232); // u8 can contain values till 255 so 1000 - (255 + 1) = 744 (Doesn't fit) so 744 - 256 = 488 (Still doesn't fit) so 488 - 256 = 232 (FITS!)

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); // -1 doesn't fit in u8 so we add -1 + (255 + 1) = 255 which is in range of u8
    
    assert_eq!(300.1_f32 as u8, 255); // 300.1 > u8::MAX
    assert_eq!(-100.1_f32 as u8, 0); // 300.1 < u8::MIN
    
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
