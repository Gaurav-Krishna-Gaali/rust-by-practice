//  Solution

fn main() {
    let c1 = 'a';
    //  char takes 4 bytes -  normal or unicode
    assert_eq!(size_of_val(&c1), 4); 

    let c2 = '中';
    // Change to 4 because unicode characters in Rust take 4 bytes
    assert_eq!(size_of_val(&c2), 4); 

    println!("Success!");
}
