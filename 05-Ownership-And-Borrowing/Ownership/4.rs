
// Solution
fn main() {
    let s = String::from("hello, world");

    // Refernce instead of var
    print_str(&s); 

    println!("{}", s);
}

fn print_str(s: &String)  {
     // Change return type to string refernce
    println!("{}",s)
}
