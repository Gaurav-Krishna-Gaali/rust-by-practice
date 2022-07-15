// Solution
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age: 24,
        hobby: String::from("procrastinating"),  // Add missing values
    };

    println!("Success!");
} 
