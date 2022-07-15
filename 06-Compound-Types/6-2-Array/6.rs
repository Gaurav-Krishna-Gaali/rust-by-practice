
// Solution
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    // Last element is either len() - 1 or 1
    let _name1 = &names[1]; 

    println!("Success!");
}
