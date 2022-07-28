// Solution
use std::collections::HashMap;
// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits

#[derive(Debug)] // Add debug trait so Viking can be printed
#[derive(Hash)] // Add hash trait 
#[derive(Eq, PartialEq)] // PartialEq and Eq traits
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}