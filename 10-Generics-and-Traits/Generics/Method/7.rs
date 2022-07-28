// Solution
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Change the num to  floating point numbers so they can access the method distance_from_origin 
    // as it is only implemented for Points with floating point numbers
    let p = Point{x: 5.0, y: 10.0}; 
    println!("{}",p.distance_from_origin());
}