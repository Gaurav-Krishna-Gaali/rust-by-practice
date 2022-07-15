// Solution
// Didnt get a solution
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(5); 
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
