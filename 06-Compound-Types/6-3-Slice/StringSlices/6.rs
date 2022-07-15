
// Solution
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // Place println! here so it doesn't give error as we clear the string.
    println!("the first word is: {}", word); 
    s.clear();
}
fn first_word(s: &str) -> &str {
    &s[..1]
}
