// Solution
// Implement the Debug trait for Structure
#[derive(Debug)] 
struct Structure(i32);

fn main() {
    // Use {} or {:?} because primitive types implement the fmt::Display trait
    println!("{:?} months in a year.", 12);

    // Because we implemented the Debug trait for Strcuture, we use :? operator
    println!("Now {:?} will print!", Structure(3)); 
}