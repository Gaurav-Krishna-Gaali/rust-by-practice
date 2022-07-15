
// Solution

fn main() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
     // 12 because 3 chars in array
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
