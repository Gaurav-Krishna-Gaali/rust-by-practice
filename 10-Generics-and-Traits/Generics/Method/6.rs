// Given

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// 
// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup
// }
// 
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
// 
//     let p3 = p1.mixup(p2);
// 
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// 
//     println!("Success!");
// }

// Solution
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<X, Y>(self, p: Point<X, Y>) -> Point<T, Y> { 
        Point {
            x: self.x, // First value of the point instance
            y: p.y, // Second value of p
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}