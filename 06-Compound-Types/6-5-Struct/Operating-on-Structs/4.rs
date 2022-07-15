
// Solution
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    // Mark p as mutable
    let mut p = Person { 
        name: String::from("sunface"),
        age,
    };

    //  sunface is only 18? 
    p.age = 30;

    // add p.name
    p.name = String::from("sunfei"); 

    println!("Success!");
}
