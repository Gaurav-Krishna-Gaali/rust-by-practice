
// Solution

fn main() {
    // Make mutable
    let mut s =  String::from("hello"); 
    s.push(','); 
     // Change to push_str because push only allows one char to be added
    s.push_str(" world");
    // Remove to_string becuase it's already a &str
    s += "!"; 

    println!("{}", s);
}

