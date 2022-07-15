// Solution
fn main() {
    let o = Some(7);

    // Add if let
    if let Some(i) = o {
        println!("This is a very long string and {:?}", i);
    }
}

