
// Solution
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0, // Change to 0
    One = 1, // Change to 1
    Two = 2, // Change to 2
}


fn main() {
    assert_eq!(Number::One as u32, Number1::One as u32); // Add as u32 to convert variant to enum
    assert_eq!(Number1::One as u32, Number2::One as u32);

    println!("Success!");
}
