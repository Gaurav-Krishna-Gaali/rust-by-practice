
// Solution
fn main() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup; // Just add the names so assert error doesn't show up.

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
