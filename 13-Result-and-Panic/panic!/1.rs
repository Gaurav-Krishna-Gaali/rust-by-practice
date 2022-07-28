// Solution
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        
        panic!("BOOOOOO"); // Panic so we don't continue
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade"); // Provide lemonade as the parameter so the program panics

    println!("Exercise Failed if printing out this line!");
}
