
// Solution
fn main() {
    let list: [i32; 100] = [1; 100] ; // Add 1 to array 100 times

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
