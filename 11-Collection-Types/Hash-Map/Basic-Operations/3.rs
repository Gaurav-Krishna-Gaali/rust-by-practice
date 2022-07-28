// Solution
use std::collections::HashMap;
fn main() {
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    // Add 100 because health doesn't already exist so we gave it a value of 100
    assert_eq!(player_stats["health"], 100);

    player_stats.entry("health").or_insert_with(random_stat_buff);
    // Health is still 100 because it already exists so we don't add the new value
    assert_eq!(player_stats["health"], 100); 

    let health = player_stats.entry("health").or_insert(50);
    // Add &100 because health already exists and because .entry() returns a reference
    assert_eq!(health, &100); 
    *health -= 50;
    // Add 50 because 100 - 50 = 50
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
}
