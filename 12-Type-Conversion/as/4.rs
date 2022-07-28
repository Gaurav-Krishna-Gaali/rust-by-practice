// Solution
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    // We convert first_address to usize(unsigned 64 in my case)
    let first_address: usize = p1 as usize; 
    // 4 == std::mem::size_of::<i32>()
    let second_address = first_address + 4; 
    // We convert it to a mutable
    let p2: *mut i32 = second_address as *mut i32; 
    unsafe {
        // add one to the second element
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}