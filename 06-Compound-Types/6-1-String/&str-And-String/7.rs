// Solution


fn main() {
    // Use String::from
    let s =  String::from("hello, world");
    greetings(s);
    
    // to_string
    let s = "hello, world".to_string();
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}

