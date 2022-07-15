// Solution
fn main() {}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"), // Add range manually using pipes
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

