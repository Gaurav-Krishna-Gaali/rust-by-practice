
// Solution
#[derive(Debug)] // Make struct printable
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        width: dbg!(30 * scale), 
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print with debug trait
}
