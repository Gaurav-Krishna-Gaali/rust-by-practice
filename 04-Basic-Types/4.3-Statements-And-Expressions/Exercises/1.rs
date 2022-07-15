
//  Solution
fn main() {
    let v = {
        let mut x = 1;
        
        //  One
        // x + 2
        
        //  Two
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}
