
// Solution
fn main() {
    // Method one
    let s: Box<str> =  "hello, world".into();
    //  turn it into &str
    greetings(&s); 
    
    // Method two
    // Change to &str
    let s: &str = "hello, world"; 
    greetings(s);
}

fn greetings(s: &str) {
    println!("{}",s)
}

