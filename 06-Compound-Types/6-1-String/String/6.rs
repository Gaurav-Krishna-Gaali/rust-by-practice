
// Solution

fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // Clone s1 , macro and we s2 a refrence
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

