// Solution

// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 // Return counter * 2, to make it 20, with break statement
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

