// Solution

// using loop to never allow the code to proceed
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    loop {
        println!("Not gonna let you get through >:D");
    }
}

