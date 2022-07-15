
// Solution
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           break; // Add break
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}


