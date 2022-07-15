
// Solution
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;
// Change f.name to _name because we moved the value and 
// remove printing of f because we cannot print the entire struct as
//  name has has been borrowed
    println!("{}, {}", _name, f.data); 
} 
