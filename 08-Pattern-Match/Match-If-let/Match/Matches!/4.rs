// Solution
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        //  matches! expression here with char ranges and number range
         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9')) 
    }

    println!("Success!");
} 

