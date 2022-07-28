// Solution
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    // Add elements of v1 to v2
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!")
}