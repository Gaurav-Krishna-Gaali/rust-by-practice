// Solution
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Add return type which is Self that refers to TrafficLight itself
    pub fn new() -> Self { 
        // Make struct, use Self instead of TrafficLight
        Self { 
            color: String::from("red"),
        }
    } 

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}
