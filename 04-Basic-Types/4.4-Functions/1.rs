
//  Solution
fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 { 
    // Add return value and add type for x
    // remove semicolon
    x + y 
}
