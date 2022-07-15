// Solution
// Add mut keyword to make x mutable and  be able to add 2 to it

fn main() {
    let (mut x, y) = (1, 2); 

    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
