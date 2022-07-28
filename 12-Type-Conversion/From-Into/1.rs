// Solution
fn main() {
    // impl From<bool> for i32
   let i1:i32 = false.into();
   let i2:i32 = i32::from(false);  
   assert_eq!(i1, i2);
   assert_eq!(i1, 0);

   // Method one
   // Use as keyword
   let i3: i32 = 'a' as i32;

   // Method two
   // Use .try_into()
   use std::convert::TryInto;
   let i3: u32 = 'a'.try_into().unwrap();

   // Method three
   // Use .into()
   let i3: u32 = 'a'.into();

   // Method One
   // Use String::from
   let s: String = String::from('a');

   // Method two
   // Use .into()
   let s: String = 'a'.into();

   println!("Success!")
}