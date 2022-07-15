
// Solution
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six { // Add some variant
        println!("{}", n);

        println!("Success!");
        
        // Add return so error doesn't throw
        return
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Add none variant
        Some(i) => Some(i + 1), // Add Some variant
    }
}
