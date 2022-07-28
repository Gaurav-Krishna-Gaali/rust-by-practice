// Solution
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69); // Change to int 69 instead of float 69.0 
    scores.insert("Katie", 58); // Change str "58" to integer 58

    // get returns a Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98)); // Add & before int because .get() returns a reference to the value not the actual value

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95); // Add 95 as the value of Daniel is 95
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3); // Scores has 4 key-value pairs but we write 3 as the len because len of a hash map starts at 0

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

