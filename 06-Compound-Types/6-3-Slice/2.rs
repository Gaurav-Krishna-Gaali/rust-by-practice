
// Solution
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    assert!(std::mem::size_of_val(&slice) == 16); // Change to 16
 
    println!("Success!");
}
