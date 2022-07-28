// Solution
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Make the function return the area
    fn area(&self) -> u32 {
        // return ares = h* w of the instance
        self.width * self.height 
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}