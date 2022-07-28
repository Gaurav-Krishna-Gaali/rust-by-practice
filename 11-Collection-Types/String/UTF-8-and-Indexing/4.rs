// Solution
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[..=0]; // Add & to make it a slice and change 0 to an inclusive range ..=0
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Change range to exclusive 7..10
    assert_eq!(slice2, "世");
    
    // iterate all chars in s
    for (i, c) in s.chars().enumerate() { // Add .chars() to get an iterateable of all the characters in the String and enumerate to get the index too
        if i == 7 { 
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}