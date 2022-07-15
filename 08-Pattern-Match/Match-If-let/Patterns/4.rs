
// Solution
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
         // Add guard to check if x is less than split
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

